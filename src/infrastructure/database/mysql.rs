use std::error::Error;

use chrono::Utc;
use mysql::prelude::Queryable;
use mysql::{Error as MySQLError, Pool};

use crate::domain::gateway::{BookManager, UserManager};
use crate::domain::model;

pub struct MySQLPersistence {
    pool: Pool,
    page_size: u32,
}

impl MySQLPersistence {
    pub fn new(dsn: &str, page_size: u32) -> Result<Self, MySQLError> {
        let pool = Pool::new(dsn)?;
        Ok(MySQLPersistence { pool, page_size })
    }
}

impl BookManager for MySQLPersistence {
    fn create_book(&self, b: &model::Book) -> Result<u32, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec::<usize, &str, (String, String, String, String, String, u32)>(
            "INSERT INTO books (title, author, published_at, description, isbn, total_pages)
             VALUES (?, ?, ?, ?, ?, ?)",
            (
                b.title.clone(),
                b.author.clone(),
                b.published_at.clone(),
                b.description.clone(),
                b.isbn.clone(),
                b.total_pages,
            ),
        )?;
        Ok(conn.last_insert_id() as u32)
    }

    fn update_book(&self, id: u32, b: &model::Book) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec::<usize, &str, (String, String, String, String, String, u32, String, u32)>(
            "UPDATE books SET title = ?, author = ?, published_at = ?, description = ?, isbn = ?, total_pages = ?, updated_at = ?
            WHERE id = ?",
            (b.title.clone(), b.author.clone(), b.published_at.clone(), b.description.clone(), b.isbn.clone(), b.total_pages, Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),  id),
        )?;
        Ok(())
    }

    fn delete_book(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec::<usize, &str, (u32,)>("DELETE FROM books WHERE id = ?", (id,))?;
        Ok(())
    }

    fn get_book(&self, id: u32) -> Result<Option<model::Book>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        let books = conn.query_map(
            format!("SELECT * FROM books WHERE ID = {}", id),
            |(
                id,
                title,
                author,
                published_at,
                description,
                isbn,
                total_pages,
                created_at,
                updated_at,
            ): (
                u64,
                String,
                String,
                String,
                String,
                String,
                u64,
                String,
                String,
            )| {
                model::Book {
                    id: id as u32,
                    title,
                    author,
                    published_at,
                    description,
                    isbn,
                    total_pages: total_pages as u32,
                    created_at,
                    updated_at,
                }
            },
        )?;
        Ok(books.first().cloned())
    }

    fn get_books(&self, offset: u32, keyword: &str) -> Result<Vec<model::Book>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        let mut query = format!("SELECT * FROM books LIMIT {}, {}", offset, self.page_size);
        if !keyword.is_empty() {
            // Warning: don't do SQL concatenation in production code. It leads to SQL Injection Vulnerabilities.
            // Here is for tutorial brevity.
            let term = format!("%{}%", keyword.replace("'", ""));
            query = format!(
                "SELECT * FROM books WHERE title LIKE '{}' OR author LIKE '{}' LIMIT {}, {}",
                term, term, offset, self.page_size
            );
        }
        let books = conn.query_map(
            query,
            |(
                id,
                title,
                author,
                published_at,
                description,
                isbn,
                total_pages,
                created_at,
                updated_at,
            ): (
                u64,
                String,
                String,
                String,
                String,
                String,
                u64,
                String,
                String,
            )| {
                model::Book {
                    id: id as u32,
                    title,
                    author,
                    published_at,
                    description,
                    isbn,
                    total_pages: total_pages as u32,
                    created_at,
                    updated_at,
                }
            },
        )?;
        Ok(books)
    }
}

impl UserManager for MySQLPersistence {
    fn create_user(&self, u: &model::User) -> Result<u32, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec::<usize, &str, (String, String, String, bool, String, String)>(
            "INSERT INTO users (email, password, salt, is_admin, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            (
                u.email.clone(),
                u.password.clone(),
                u.salt.clone(),
                u.is_admin,
                u.created_at.clone(),
                u.updated_at.clone(),
            ),
        )?;
        Ok(conn.last_insert_id() as u32)
    }

    fn get_user_by_email(&self, email: &str) -> Result<Option<model::User>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        let users = conn.query_map(
            format!(
                "SELECT * FROM users WHERE email = '{}'",
                email.replace("'", "")
            ),
            |(id, email, password, salt, is_admin, created_at, updated_at): (
                u64,
                String,
                String,
                String,
                bool,
                String,
                String,
            )| {
                model::User {
                    id: id as u32,
                    email,
                    password,
                    salt,
                    is_admin,
                    created_at,
                    updated_at,
                }
            },
        )?;
        Ok(users.first().cloned())
    }
}
