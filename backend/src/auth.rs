use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
}

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        AuthService { pool }
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        sqlx::query("INSERT INTO users (username, password_hash, salt) VALUES ($1, $2, $3)")
            .bind(username)
            .bind(password_hash)
            .bind(salt.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                println!("ERROR: {}", e);
                Error::Password
            })?;

        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Password)?;

        match row {
            Some(user) => Ok(User {
                id: user.id,
                username: user.username,
                password_hash: String::new(),
                salt: String::new(),
            }),
            None => Err(Error::Password),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<Option<i32>, Error> {
        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| Error::Password)?;

        if let Some(user) = row {
            let argon2 = Argon2::default();
            let hash = PasswordHash::new(&user.password_hash).unwrap();
            if argon2.verify_password(password.as_bytes(), &hash).is_ok() {
                return Ok(Some(user.id));
            }
        }
        Ok(None)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn deregister(&self, id: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
