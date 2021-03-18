pub mod serializer;

use std::io::{prelude::*, Cursor};

use anyhow::Result;
use varint::VarintWrite; // VarintRead

// TODO move all blockchain-related types to hydra-proto after adding typetags to Asset and TransactionType.
/// State identifier of a decentralized ledger, usually represented as a sequence number of blocks.
pub type BlockHeight = u32;
/// Duration (e.g. a year) expressed as an expected number of blocks on the ledger, approximating the duration.  
pub type BlockCount = u32;
/// A unique value attached to requests as protection from replay attacks.
pub type Nonce = u64;
