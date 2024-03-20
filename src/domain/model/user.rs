#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}
