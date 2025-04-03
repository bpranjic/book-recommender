use neo4rs::*;
use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    id: String,
    title: String,
    author: String,
    genre: String,
    cover: String,
}

impl Book {
    pub fn new(id: String, title: String, author: String, genre: String, cover: String) -> Self {
        Self {
            id,
            title,
            author,
            genre,
            cover,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn genre(&self) -> &String {
        &self.genre
    }

    pub fn cover(&self) -> &String {
        &self.cover
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct DatabaseService {
    graph: Graph,
}

impl DatabaseService {
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self, Error> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .fetch_size(500)
            .build()
            .unwrap();
        let graph = Graph::connect(config)
            .await
            .expect("Could not connect to Neo4j Aura instance.");
        Ok(Self { graph })
    }

    pub async fn get_all_books(&self) -> Vec<Book> {
        let mut result = self
            .graph
            .execute(query(
                "MATCH (b:Book) RETURN b.id, b.title, b.author, b.genre, b.cover",
            ))
            .await
            .map_err(|e| -> Vec<Book> {
                println!("Could not fetch books: {}", e);
                return vec![];
            })
            .unwrap();

        let mut books: Vec<Book> = vec![];
        while let Ok(Some(row)) = result.next().await {
            books.push(Book::new(
                row.get("b.id").unwrap_or_default(),
                row.get("b.title").unwrap_or_default(),
                row.get("b.author").unwrap_or_default(),
                row.get("b.genre").unwrap_or_default(),
                row.get("b.cover").unwrap_or_default(),
            ));
        }
        books
    }

    pub async fn get_book(&self, id: &str) -> Option<Book> {
        let mut result = self
            .graph
            .execute(
                query("MATCH (b:Book {id: $id}) RETURN b.id, b.title, b.author, b.genre, b.cover")
                    .param("id", id),
            )
            .await
            .map_err(|e| -> Option<Book> {
                println!("Could not fetch book with id '{}': {}", id, e);
                return None;
            })
            .unwrap();

        while let Ok(Some(row)) = result.next().await {
            return Some(Book::new(
                row.get("b.id").unwrap_or_default(),
                row.get("b.title").unwrap_or_default(),
                row.get("b.author").unwrap_or_default(),
                row.get("b.genre").unwrap_or_default(),
                row.get("b.cover").unwrap_or_default(),
            ));
        }
        None
    }

    pub async fn add_book(&self, book: &Book) -> Option<String> {
        let mut result = self
            .graph
            .execute(
                query("CREATE (b:Book {id: $id, title: $title, author: $author, genre: $genre, cover: $cover}) RETURN elementId(b) AS id")
                    .param("id", book.id.as_str())
                    .param("title", book.title.as_str())
                    .param("author", book.author.as_str())
                    .param("genre", book.genre.as_str())
                    .param("cover", book.cover.as_str())
            )
            .await
            .map_err(|e| -> Option<String> {
                println!("Could not add book: {}", e);
                None
            })
            .unwrap();

        while let Ok(Some(row)) = result.next().await {
            return row.get("id").ok();
        }
        None
    }

    pub async fn edit_book(&self, id: &str, book: &Book) -> Result<()> {
        let _ = self
            .graph
            .run(
                query(
                    "MATCH (b:Book {id: $id}) 
                    SET b.title = $title, b.author = $author, b.genre = $genre, b.cover = $cover",
                )
                .param("id", id)
                .param("title", book.title.as_str())
                .param("author", book.author.as_str())
                .param("genre", book.genre.as_str())
                .param("cover", book.cover.as_str()),
            )
            .await
            .map_err(|e| {
                println!("Could not update book with id '{}': {}", id, e);
                e
            })?;
        Ok(())
    }

    pub async fn delete_book(&self, id: &str) -> Result<()> {
        let _ = self
            .graph
            .run(query("MATCH (b:Book {id: $id}) DETACH DELETE b").param("id", id))
            .await
            .map_err(|e| {
                println!("Could not delete book with id '{}': {}", id, e);
                e
            })?;
        Ok(())
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        let mut result = self
            .graph
            .execute(query("MATCH (u:User) RETURN u.id, u.name"))
            .await
            .map_err(|e| -> Vec<User> {
                println!("Could not fetch users: {}", e);
                return vec![];
            })
            .unwrap();

        let mut users: Vec<User> = vec![];
        while let Ok(Some(row)) = result.next().await {
            users.push(User::new(
                row.get("u.id").unwrap_or_default(),
                row.get("u.name").unwrap_or_default(),
            ));
        }
        users
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        let mut result = self
            .graph
            .execute(query("MATCH (u:User {id: $id}) RETURN u.id, u.name").param("id", id))
            .await
            .map_err(|e| -> Option<User> {
                println!("Could not fetch user with id '{}': {}", id, e);
                return None;
            })
            .unwrap();

        while let Ok(Some(row)) = result.next().await {
            return Some(User::new(
                row.get("u.id").unwrap_or_default(),
                row.get("u.name").unwrap_or_default(),
            ));
        }
        None
    }

    pub async fn add_user(&self, user: &User) -> Option<String> {
        let mut result = self
            .graph
            .execute(
                query("CREATE (u:User {id: $id, name: $name}) RETURN elementId(u) AS id")
                    .param("id", user.id)
                    .param("name", user.name.as_str()),
            )
            .await
            .map_err(|e| -> Option<String> {
                println!("Could not add user: {}", e);
                None
            })
            .unwrap();

        while let Ok(Some(row)) = result.next().await {
            return row.get("id").ok();
        }
        None
    }

    pub async fn edit_user(&self, id: i32, user: &User) -> Result<()> {
        let _ = self
            .graph
            .run(
                query("MATCH (u:User {id: $id}) SET u.name = $name")
                    .param("id", id)
                    .param("name", user.name.as_str()),
            )
            .await
            .map_err(|e| {
                println!("Could not update user with id '{}': {}", id, e);
                e
            })?;
        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<()> {
        let _ = self
            .graph
            .run(query("MATCH (u:User {id: $id}) DETACH DELETE u").param("id", id))
            .await
            .map_err(|e| {
                println!("Could not delete user with id '{}': {}", id, e);
                e
            })?;
        Ok(())
    }

    pub async fn get_user_books(&self, id: i32) -> Vec<Book> {
        let mut result = self
            .graph
            .execute(query(
                "MATCH (u:User {id: $id})-[:HAS_READ]->(b:Book) RETURN b.id, b.title, b.author, b.genre, b.cover",
            )
            .param("id", id))
            .await
            .map_err(|e| -> Vec<Book> {
                println!("Could not fetch books for user with id '{}': {}", id, e);
                return vec![];
            })
            .unwrap();

        let mut books: Vec<Book> = vec![];
        while let Ok(Some(row)) = result.next().await {
            books.push(Book::new(
                row.get("b.id").unwrap_or_default(),
                row.get("b.title").unwrap_or_default(),
                row.get("b.author").unwrap_or_default(),
                row.get("b.genre").unwrap_or_default(),
                row.get("b.cover").unwrap_or_default(),
            ));
        }
        books
    }

    pub async fn add_book_to_user(&self, user_id: i32, book_id: &str) -> Result<()> {
        let _ = self
            .graph
            .run(query("MATCH (u:User {id: $user_id}) MATCH (b:Book {id: $book_id}) MERGE (u)-[r:HAS_READ]->(b)")
                .param("user_id", user_id)
                .param("book_id", book_id)
            )
            .await
            .map_err(|e| {
                println!("Could not add book with id '{}' to user with id '{}': {}", book_id, user_id, e);
                e
            })?;
        Ok(())
    }

    pub async fn remove_book_from_user(&self, user_id: i32, book_id: &str) -> Result<()> {
        let _ = self
            .graph
            .run(
                query(
                    "MATCH (u:User {id: $user_id})-[r:HAS_READ]->(b:Book {id: $book_id}) DELETE r",
                )
                .param("user_id", user_id)
                .param("book_id", book_id),
            )
            .await
            .map_err(|e| {
                println!(
                    "Could not remove book with id '{}' from user with id '{}': {}",
                    book_id, user_id, e
                );
                e
            })?;
        Ok(())
    }

    pub async fn recommend_books(&self, id: i32) -> Vec<Book> {
        let mut result = self
            .graph
            .execute(query(
                "MATCH (u:User {id: $id})-[:HAS_READ]->(b:Book)
                    MATCH (similarUser:User)-[:HAS_READ]->(b)
                    WHERE similarUser <> u
                    MATCH (similarUser)-[:HAS_READ]->(rec:Book)
                    WHERE NOT (u)-[:HAS_READ]->(rec)
                    RETURN rec.id, rec.title, rec.author, rec.genre, rec.cover, COUNT(similarUser) AS score
                    ORDER BY score DESC
                    LIMIT 5",
            )
            .param("id", id))
            .await
            .map_err(|e| -> Vec<Book> {
                println!("Could not fetch recommended books for user with id '{}': {}", id, e);
                return vec![];
            })
            .unwrap();

        let mut books: Vec<Book> = vec![];
        while let Ok(Some(row)) = result.next().await {
            books.push(Book::new(
                row.get("rec.id").unwrap_or_default(),
                row.get("rec.title").unwrap_or_default(),
                row.get("rec.author").unwrap_or_default(),
                row.get("rec.genre").unwrap_or_default(),
                row.get("rec.cover").unwrap_or_default(),
            ));
        }
        books
    }
}
