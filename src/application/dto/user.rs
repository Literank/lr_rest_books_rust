#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserCredential {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub email: String,
}
