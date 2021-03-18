mod alg;
mod token;

pub use alg::JwtMultiCipher;
pub use token::{JwtBuilder, JwtParser};

use std::convert::TryFrom;

use chrono::{DateTime, Duration, Utc};
use jwt_compact::{prelude::*, Algorithm, AlgorithmSignature, Token};

use iop_keyvault::{
    multicipher::{MPrivateKey, MPublicKey, MSignature},
    PrivateKey as _, PublicKey as _,
};

use super::*;
