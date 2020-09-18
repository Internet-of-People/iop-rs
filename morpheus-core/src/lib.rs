pub mod crypto;
pub mod data;
pub mod hydra;

// imports from standard library

use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::rc::Rc;
use std::str::FromStr;

// imports from 3rd party crates

use anyhow::{anyhow, bail, ensure, Result};
use log::*;
use serde::{Deserialize, Serialize};

use json_digest::*;

use data::{auth::Authentication, did::Did};
