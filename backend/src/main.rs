mod auth;
mod database;

use std::option;

use anyhow::Context;
use auth::AuthService;
use database::{Book, DatabaseService, User};
use futures::lock::Mutex;
use rocket::{
    delete,
    fairing::{Fairing, Info, Kind},
    form::Form,
    get,
    http::Header,
    options, post, put, routes,
    serde::json::Json,
    FromForm, Request, Response, State,
};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, DELETE, PATCH, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == rocket::http::Method::Options {
            response.set_status(rocket::http::Status::NoContent);
            response.set_header(Header::new("Content-Length", "0"));
        }
    }
}

#[derive(FromForm)]
struct UserRequest {
    #[field(name = "username")]
    username: String,
    #[field(name = "password")]
    password: String,
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[options("/auth/users")]
async fn options_auth_users() -> &'static str {
    ""
}

#[get("/auth/users")]
async fn get_auth_users(auth_service: &State<Mutex<AuthService>>) -> Json<Vec<auth::User>> {
    tracing::info!("TRACING");
    let auth_service = auth_service.lock().await;
    match auth_service.get_all_users().await {
        Ok(users) => Json(users),
        Err(e) => {
            println!("Could not get auth users: {}", e);
            Json(vec![])
        }
    }
}

#[options("/login")]
async fn options_login() -> &'static str {
    ""
}

#[post("/login", data = "<user_form>")]
async fn login(
    user_form: Form<UserRequest>,
    auth_service: &State<Mutex<AuthService>>,
) -> Json<Option<auth::User>> {
    tracing::info!("TRACING");
    let auth_service = auth_service.lock().await;
    match auth_service
        .login(&user_form.username, &user_form.password)
        .await
    {
        Ok(Some(id)) => Json(Some(auth::User {
            id: id,
            username: user_form.username.clone(),
            password_hash: String::new(),
            salt: String::new(),
        })),
        Ok(None) => Json(None),
        Err(_) => Json(None),
    }
}

#[options("/register")]
async fn options_register() -> &'static str {
    ""
}

#[post("/register", data = "<user_form>")]
async fn register(
    user_form: Form<UserRequest>,
    auth_service: &State<Mutex<AuthService>>,
) -> Json<Option<auth::User>> {
    tracing::info!("TRACING");
    let auth_service = auth_service.lock().await;
    match auth_service
        .register_user(&user_form.username, &user_form.password)
        .await
    {
        Ok(user) => Json(Some(user)),
        Err(e) => {
            println!("Failed to register user: {}", e);
            Json(None)
        }
    }
}

#[options("/auth/users/<id>")]
async fn options_auth_users_id(id: i32) -> &'static str {
    ""
}

#[delete("/auth/users/<id>")]
async fn deregister(id: i32, auth_service: &State<Mutex<AuthService>>) -> Json<bool> {
    tracing::info!("TRACING");
    let auth_service = auth_service.lock().await;
    Json(auth_service.deregister(id).await.is_ok())
}

#[options("/books")]
async fn options_books() -> &'static str {
    ""
}

#[get("/books")]
async fn get_all_books(database_service: &State<Mutex<DatabaseService>>) -> Json<Vec<Book>> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.get_all_books().await)
}

#[options("/books/<id>")]
async fn options_books_id(id: &str) -> &'static str {
    ""
}

#[get("/books/<id>")]
async fn get_book(
    id: &str,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<Option<Book>> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.get_book(&id).await)
}

#[post("/books", format = "application/json", data = "<book>")]
async fn add_book(
    book: Json<Book>,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<String> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    match database_service.add_book(&book).await {
        Some(id) => Json(id),
        None => Json(String::new()),
    }
}

#[put("/books/<id>", data = "<book>")]
async fn update_book(
    id: &str,
    book: Json<Book>,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<bool> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.edit_book(id, &book).await.is_ok())
}

#[delete("/books/<id>")]
async fn delete_book(id: &str, database_service: &State<Mutex<DatabaseService>>) -> Json<bool> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.delete_book(id).await.is_ok())
}

#[get("/users")]
async fn get_all_users(database_service: &State<Mutex<DatabaseService>>) -> Json<Vec<User>> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.get_all_users().await)
}

#[options("/users/<id>")]
async fn options_users_id(id: i32) -> &'static str {
    ""
}

#[get("/users/<id>")]
async fn get_user(id: i32, database_service: &State<Mutex<DatabaseService>>) -> Json<Option<User>> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.get_user(id).await)
}

#[options("/users")]
fn options_users() -> &'static str {
    ""
}

#[post("/users", format = "application/json", data = "<user>")]
async fn add_user(
    user: Json<User>,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<String> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    match database_service.add_user(&user).await {
        Some(id) => Json(id),
        None => Json(String::new()),
    }
}

#[put("/users/<id>", data = "<user>")]
async fn update_user(
    id: i32,
    user: Json<User>,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<bool> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.edit_user(id, &user).await.is_ok())
}

#[delete("/users/<id>")]
async fn delete_user(id: i32, database_service: &State<Mutex<DatabaseService>>) -> Json<bool> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.delete_user(id).await.is_ok())
}

#[options("/users/<id>/books")]
async fn options_users_id_books(id: i32) -> &'static str {
    ""
}

#[get("/users/<id>/books")]
async fn get_user_books(
    id: i32,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<Vec<Book>> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.get_user_books(id).await)
}

#[post(
    "/users/<user_id>/books",
    format = "application/json",
    data = "<book_id>"
)]
async fn add_book_to_user(
    user_id: i32,
    book_id: &str,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<bool> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(
        database_service
            .add_book_to_user(user_id, book_id)
            .await
            .is_ok(),
    )
}

#[delete(
    "/users/<user_id>/books",
    format = "application/json",
    data = "<book_id>"
)]
async fn remove_book_from_user(
    user_id: i32,
    book_id: &str,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<bool> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(
        database_service
            .remove_book_from_user(user_id, book_id)
            .await
            .is_ok(),
    )
}

#[options("/users/<id>/recommendations")]
async fn options_users_id_recommendations(id: i32) -> &'static str {
    ""
}

#[get("/users/<id>/recommendations")]
async fn get_book_recommendations(
    id: i32,
    database_service: &State<Mutex<DatabaseService>>,
) -> Json<Vec<Book>> {
    tracing::info!("TRACING");
    let database_service = database_service.lock().await;
    Json(database_service.recommend_books(id).await)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username TEXT UNIQUE NOT NULL,
        password_hash TEXT NOT NULL,
        salt TEXT NOT NULL)",
    )
    .execute(&pool)
    .await
    .expect("COULD NOT CREATE TABLE");
    let auth_service = Mutex::new(AuthService::new(pool.clone()));
    let neo4j_uri = secrets
        .get("NEO4J_URI")
        .context("NEO4J URI NOT FOUND.")
        .unwrap();
    let neo4j_user = secrets
        .get("NEO4J_USERNAME")
        .context("NEO4J USERNAME NOT FOUND.")
        .unwrap();
    let neo4j_password = secrets
        .get("NEO4J_PASSWORD")
        .context("NEO4J PASSWORD NOT FOUND.")
        .unwrap();
    let database_service = Mutex::new(
        DatabaseService::new(&neo4j_uri, &neo4j_user, &neo4j_password)
            .await
            .expect("Failed to connect to Neo4j instance"),
    );
    let rocket = rocket::build()
        .attach(CORS)
        .manage(auth_service)
        .manage(database_service)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                options_auth_users,
                get_auth_users,
                options_login,
                login,
                options_register,
                register,
                options_auth_users_id,
                deregister,
                options_books,
                get_all_books,
                options_books_id,
                get_book,
                add_book,
                update_book,
                delete_book,
                get_all_users,
                options_users_id,
                get_user,
                options_users,
                add_user,
                update_user,
                delete_user,
                options_users_id_books,
                get_user_books,
                add_book_to_user,
                remove_book_from_user,
                options_users_id_recommendations,
                get_book_recommendations
            ],
        );

    Ok(rocket.into())
}
