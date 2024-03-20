use std::error::Error;
use std::time::{Duration, SystemTime};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::gateway::PermissionManager;
use crate::domain::model;

// Keeper manages user tokens.
pub struct Keeper {
    secret_key: String,
    expire_hours: u64,
}

// UserClaims includes user info.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    user_id: u32,
    user_name: String,
    permission: model::UserPermission,
    exp: usize, // Expiry time in seconds since epoch
}

impl Keeper {
    // NewTokenKeeper constructs a new JWT token keeper
    pub fn new(secret_key: String, expire_in_hours: u32) -> Self {
        Keeper {
            secret_key,
            expire_hours: expire_in_hours as u64,
        }
    }

    // extract_token extracts the token from the signed string.
    fn extract_token(&self, token_result: &str) -> Result<UserClaims, Box<dyn Error>> {
        let token_data = decode::<UserClaims>(
            token_result,
            &DecodingKey::from_secret(self.secret_key.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

impl PermissionManager for Keeper {
    // generate_token generates a new JWT token.
    fn generate_token(
        &self,
        user_id: u32,
        email: &str,
        perm: model::UserPermission,
    ) -> Result<String, Box<dyn Error>> {
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(self.expire_hours * 3600))
            .ok_or("Overflow when adding expire time")?;
        let exp = exp.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as usize;

        let claims = UserClaims {
            user_id,
            user_name: email.to_owned(),
            permission: perm,
            exp,
        };

        let header = Header::default();
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_ref()),
        )?;
        Ok(token)
    }

    // has_permission checks if user has the given permission.
    fn has_permission(
        &self,
        token_result: &str,
        perm: model::UserPermission,
    ) -> Result<bool, Box<dyn Error>> {
        let claims = self.extract_token(token_result)?;
        Ok(claims.permission >= perm)
    }
}
