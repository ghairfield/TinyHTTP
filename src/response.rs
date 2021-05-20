//! HTTP Response Handler
//!
//! Greg Hairfield
//! CS410P Rust Programming
//! Spring 2021
//!
//! # Examples
//! ```
//! TODO
//! ```

use std::collections::HashMap;
use crate::protocol::*;

pub struct Response {
    status: StatusCode,
    version: RequestVersion,
    fields: HashMap<String, String>,
}


