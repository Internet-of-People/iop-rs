use failure::Fallible;
use serde::Serialize;

use crate::crypto::json_digest::json_digest;

pub type ContentId = String;

pub trait Content: Serialize + Clone + Sized {
    fn content_id(&self) -> Fallible<ContentId> {
        let hash = json_digest(self)?;
        Ok(hash)
    }

    fn validate_id(&self, content_id: &ContentId) -> Fallible<bool> {
        let calculated_hash = self.content_id()?;
        Ok(calculated_hash == *content_id)
    }
}

impl Content for &str {}
impl Content for String {}
impl Content for serde_json::Value {}
