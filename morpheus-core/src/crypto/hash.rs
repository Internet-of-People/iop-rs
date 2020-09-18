use super::*;

pub fn hash_bytes(content: &[u8]) -> String {
    format!("ck{}", default_hasher(content))
}

pub type ContentId = String;

pub trait Content: Serialize + Clone + Sized {
    fn content_id(&self) -> Result<ContentId> {
        digest_data(self)
    }

    fn validate_id(&self, content_id: &ContentId) -> Result<bool> {
        let calculated_hash = self.content_id()?;
        Ok(calculated_hash == *content_id)
    }
}

impl Content for serde_json::Value {}

impl Content for Box<[u8]> {
    fn content_id(&self) -> Result<ContentId> {
        Ok(hash_bytes(self.as_ref()))
    }
}

impl Content for Vec<u8> {
    fn content_id(&self) -> Result<ContentId> {
        Ok(hash_bytes(self.as_ref()))
    }
}
