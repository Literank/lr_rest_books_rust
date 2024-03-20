use std::error::Error;
use std::sync::Arc;

use rand::{thread_rng, Rng};
use sha1::{Digest, Sha1};

use crate::application::dto;
use crate::domain::{gateway, model};

const SALT_LEN: usize = 4;
const ERR_EMPTY_EMAIL: &str = "empty email";
const ERR_EMPTY_PASSWORD: &str = "empty password";

pub struct UserOperator {
    user_manager: Arc<dyn gateway::UserManager>,
    perm_manager: Arc<dyn gateway::PermissionManager>,
}

impl UserOperator {
    pub fn new(u: Arc<dyn gateway::UserManager>, p: Arc<dyn gateway::PermissionManager>) -> Self {
        UserOperator {
            user_manager: u,
            perm_manager: p,
        }
    }

    pub fn create_user(&self, uc: &dto::UserCredential) -> Result<dto::User, Box<dyn Error>> {
        if uc.email.is_empty() {
            return Err(ERR_EMPTY_EMAIL.into());
        }
        if uc.password.is_empty() {
            return Err(ERR_EMPTY_PASSWORD.into());
        }
        let salt = random_string(SALT_LEN);
        let user = model::User {
            id: 0,
            email: uc.email.clone(),
            password: sha1_hash(&(uc.password.clone() + &salt)),
            salt,
            is_admin: false,
            created_at: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string(),
            updated_at: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string(),
        };
        let uid = self.user_manager.create_user(&user)?;
        Ok(dto::User {
            id: uid,
            email: uc.email.clone(),
        })
    }

    pub fn sign_in(&self, email: &str, password: &str) -> Result<dto::UserToken, Box<dyn Error>> {
        if email.is_empty() {
            return Err(ERR_EMPTY_EMAIL.into());
        }
        if password.is_empty() {
            return Err(ERR_EMPTY_PASSWORD.into());
        }
        let user = self.user_manager.get_user_by_email(email)?;
        if let Some(u) = user {
            let password_hash = sha1_hash(&(password.to_string() + &u.salt));
            if u.password != password_hash {
                return Err("wrong password".into());
            }
            let perm = if u.is_admin {
                model::UserPermission::PermAdmin
            } else {
                model::UserPermission::PermUser
            };
            let token = self.perm_manager.generate_token(u.id, &u.email, perm)?;
            Ok(dto::UserToken {
                user: dto::User {
                    id: u.id,
                    email: u.email,
                },
                token,
            })
        } else {
            Err("user does not exist".into())
        }
    }

    pub fn has_permission(
        &self,
        token: &str,
        perm: model::UserPermission,
    ) -> Result<bool, Box<dyn Error>> {
        self.perm_manager.has_permission(token, perm)
    }
}

fn random_string(length: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.gen::<usize>() % charset.len())
        .map(|idx| charset.chars().nth(idx).unwrap())
        .collect()
}

fn sha1_hash(input: &str) -> String {
    let mut h = Sha1::new();
    h.update(input);
    let hash_bytes = h.finalize();
    hex::encode(hash_bytes)
}
