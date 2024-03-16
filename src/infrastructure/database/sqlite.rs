use std::error::Error;
use std::sync::Mutex;

use chrono::Utc;
use rusqlite::{params, Connection, Result as RusqliteResult};

use crate::domain::gateway::BookManager;
use crate::domain::model;

pub struct SQLitePersistence {
    conn: Mutex<Connection>,
}

impl SQLitePersistence {
    pub fn new(file_name: &str) -> RusqliteResult<Self> {
        let conn = Connection::open(file_name)?;
        Ok(SQLitePersistence {
            conn: Mutex::new(conn),
        })
    }
}

impl BookManager for SQLitePersistence {
    fn create_book(&self, b: &model::Book) -> Result<u32, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO books (title, author, published_at, description, isbn, total_pages)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                b.title,
                b.author,
                b.published_at,
                b.description,
                b.isbn,
                b.total_pages,
            ],
        )?;
        Ok(conn.last_insert_rowid() as u32)
    }

    fn update_book(&self, id: u32, b: &model::Book) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE books SET title = ?, author = ?, published_at = ?, description = ?, isbn = ?, total_pages = ?, updated_at = ?
             WHERE id = ?",
            params![
                b.title,
                b.author,
                b.published_at,
                b.description,
                b.isbn,
                b.total_pages,
                Utc::now().to_rfc3339(),
                id,
            ],
        )?;
        Ok(())
    }

    fn delete_book(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM books WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn get_book(&self, id: u32) -> Result<Option<model::Book>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM books WHERE id = ?1")?;
        let book_iter = stmt.query_map(params![id], |row| {
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

        for result in book_iter {
            return Ok(Some(result?));
        }
        Ok(None)
    }

    fn get_books(&self) -> Result<Vec<model::Book>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM books")?;
        let book_iter = stmt.query_map([], |row| {
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
        for result in book_iter {
            books.push(result?);
        }
        Ok(books)
    }
}
