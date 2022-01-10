use super::*;

/// Multibase-encoded hash of the provided binary content, prefixed with "cb".
/// Character 'b' marks that binary content was hashed and 'c' stands for content hash.
pub fn hash_bytes(content: &[u8]) -> String {
    format!("cb{}", default_hasher(content))
}

pub type ContentId = String;

pub trait Content: Serialize + Clone + Sized {
    fn content_id(&self) -> Result<ContentId> {
        digest_data(self)
    }

    fn validate_id(&self, content_id: impl Deref<Target = ContentId>) -> Result<bool> {
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
