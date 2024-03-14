#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub published_at: String,
    pub description: String,
    pub isbn: String,
    pub total_pages: u32,
    pub created_at: String,
    pub updated_at: String,
}
