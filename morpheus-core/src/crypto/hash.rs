use failure::Fallible;
use serde::Serialize;

use crate::crypto::json_digest::{hasher, json_digest};

pub fn hash_bytes(content: &[u8]) -> String {
    format!("ck{}", hasher(content))
}

pub type ContentId = String;

pub trait Content: Serialize + Clone + Sized {
    fn content_id(&self) -> Fallible<ContentId> {
        json_digest(self)
    }

    fn validate_id(&self, content_id: &ContentId) -> Fallible<bool> {
        let calculated_hash = self.content_id()?;
        Ok(calculated_hash == *content_id)
    }
}

impl Content for serde_json::Value {}

impl Content for Box<[u8]> {
    fn content_id(&self) -> Fallible<ContentId> {
        Ok(hash_bytes(self.as_ref()))
    }
}

impl Content for Vec<u8> {
    fn content_id(&self) -> Fallible<ContentId> {
        Ok(hash_bytes(self.as_ref()))
    }
}
