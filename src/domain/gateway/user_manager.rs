use std::error::Error;

use crate::domain::model;

pub trait UserManager: Send + Sync {
    fn create_user(&self, u: &model::User) -> Result<u32, Box<dyn Error>>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<model::User>, Box<dyn Error>>;
}

pub trait PermissionManager: Send + Sync {
    fn generate_token(
        &self,
        user_id: u32,
        email: &str,
        perm: model::UserPermission,
    ) -> Result<String, Box<dyn Error>>;

    fn has_permission(
        &self,
        token: &str,
        perm: model::UserPermission,
    ) -> Result<bool, Box<dyn Error>>;
}
