#[macro_use]
extern crate rocket;

mod model;

use chrono::Utc;
use rocket::http::Status;
use rocket::response::content;
use rocket::response::status::{self, NoContent};
use rocket::serde::json::Json;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::sync::Mutex;

// Initialize a database instance
lazy_static::lazy_static! {
    static ref DB: Mutex<Database> = Mutex::new(Database::new().unwrap());
}

// Define the database schema
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> SqliteResult<Self> {
        let conn = Connection::open("test.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                published_at TEXT NOT NULL,
                description TEXT NOT NULL,
                isbn TEXT NOT NULL,
                total_pages INTEGER NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn get_books(&self) -> SqliteResult<Vec<model::Book>> {
        let mut stmt = self.conn.prepare("SELECT * FROM books")?;
        let rows = stmt.query_map([], |row| {
            Ok(model::Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                published_at: row.get(3)?,
                description: row.get(4)?,
                isbn: row.get(5)?,
                total_pages: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        let mut books = Vec::new();
        for book in rows {
            books.push(book?);
        }
        Ok(books)
    }

    pub fn get_book(&self, id: u32) -> SqliteResult<Option<model::Book>> {
        let mut stmt = self.conn.prepare("SELECT * FROM books WHERE id = ?")?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(model::Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                published_at: row.get(3)?,
                description: row.get(4)?,
                isbn: row.get(5)?,
                total_pages: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_book(&self, book: &model::Book) -> SqliteResult<()> {
        self.conn.execute(
            "INSERT INTO books (title, author, published_at, description, isbn, total_pages)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                book.title,
                book.author,
                book.published_at,
                book.description,
                book.isbn,
                book.total_pages,
            ],
        )?;
        Ok(())
    }

    pub fn update_book(&self, id: u32, book: &model::Book) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE books SET title = ?, author = ?, published_at = ?, description = ?, isbn = ?, total_pages = ?, updated_at = ?
             WHERE id = ?",
            params![
                book.title,
                book.author,
                book.published_at,
                book.description,
                book.isbn,
                book.total_pages,
                Utc::now().to_rfc3339(),
                id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_book(&self, id: u32) -> SqliteResult<()> {
        self.conn.execute("DELETE FROM books WHERE id = ?", [id])?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

// Define a health endpoint handler, use `/health` or `/`
#[get("/")]
fn health() -> content::RawJson<&'static str> {
    // Return a simple response indicating the server is healthy
    content::RawJson("{\"status\":\"ok\"}")
}

#[get("/books")]
fn get_books() -> Result<Json<Vec<model::Book>>, status::Custom<Json<ErrorResponse>>> {
    let db = DB.lock().unwrap();
    match db.get_books() {
        Ok(books) => Ok(Json(books)),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[get("/books/<id>")]
fn get_book(id: u32) -> Result<Json<model::Book>, status::Custom<Json<ErrorResponse>>> {
    let db = DB.lock().unwrap();
    match db.get_book(id) {
        Ok(book) => match book {
            Some(b) => Ok(Json(b)),
            None => Err(status::Custom(
                Status::NotFound,
                Json(ErrorResponse {
                    error: format!("book {id} not found"),
                }),
            )),
        },
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[post("/books", format = "json", data = "<book>")]
fn create_book(
    book: Json<model::Book>,
) -> Result<Json<model::Book>, status::Custom<Json<ErrorResponse>>> {
    let db = DB.lock().unwrap();
    match db.create_book(&book) {
        Ok(_) => Ok(book),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[put("/books/<id>", format = "json", data = "<book>")]
fn update_book(
    id: u32,
    book: Json<model::Book>,
) -> Result<Json<model::Book>, status::Custom<Json<ErrorResponse>>> {
    let db = DB.lock().unwrap();
    match db.update_book(id, &book) {
        Ok(_) => Ok(book),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[delete("/books/<id>")]
fn delete_book(id: u32) -> Result<NoContent, status::Custom<Json<ErrorResponse>>> {
    let db = DB.lock().unwrap();
    match db.delete_book(id) {
        Ok(_) => Ok(NoContent),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            health,
            get_books,
            get_book,
            create_book,
            update_book,
            delete_book
        ],
    )
}
