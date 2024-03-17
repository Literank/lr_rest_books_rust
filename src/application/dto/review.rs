#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReviewBody {
    pub book_id: u32,
    pub author: String,
    pub title: String,
    pub content: String,
}
