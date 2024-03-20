// UserPermission represents different levels of user permissions.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum UserPermission {
    PermNone,
    PermUser,
    PermAuthor,
    PermAdmin,
}

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
