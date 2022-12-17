#![feature(trace_macros)]
#![allow(unused_imports, unreachable_code)]
#![allow(unused_variables, dead_code, unused_must_use)]

use std::alloc;

pub mod serialization;
pub use crate::serialization::*;

mod errors;
pub use crate::errors::*;

mod client;
pub use crate::client::*;
