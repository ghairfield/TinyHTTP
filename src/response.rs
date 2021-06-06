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
use crate::request;

static HTTP_PAYLOAD: &str = r##"
    <!DOCTYPE html>
    <html>
        <body>
            <h1>Hello World!!</h1>
            <p>This is a response from TinyHTTP</p>
        </body>
    </html>
    "##;

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

/// The Response for a request.
/// Unlike a request, the values here can be changed by a user to specify
/// a response to a request. This is similar behavior to Python Flask and
/// Node Express.
pub struct Response {
    pub status: StatusCode,
    pub version: RequestVersion,
    pub fields: HashMap<String, String>,
    pub payload: String,
}

impl Response {
    /// Create a new Response structure with some default values. 
    pub fn new() -> Self {
        Response {
            status:  StatusCode::Unknown,
            version: RequestVersion::HTTP1,
            fields:  HashMap::<String, String>::new(),
            payload: String::new()
        }
    }

    /// Build a response for the client. 
    pub fn create_response(&mut self, h: &request::Header) -> Result<(), ResponseError>{
        if !h.is_valid() {
            self.status = StatusCode::BadRequest;
            return Ok(());
        }

        self.status = StatusCode::OK;
        self.payload = HTTP_PAYLOAD.to_string();
        Ok(())
    }

    pub fn to_network(&self) -> String {
        /*
        let mut r: Vec::<u8>::new();
        r.push(version_to_string(self.version).as_bytes().to_vec());
        r.push(b' ');
        r.push(status_to_string(self.status).as_bytes().to_vec());
        r.push(b"\r\n");
        r.push(b"Content-Length: ");
        r.push(self.payload.len().to_string().as_bytes().to_vec());
        r.push(b"\r\n\r\n");
        r.push(self.payload.clone());
        */
        let r = format!{
            "{} {}\r\nContent-Lenght: {}\r\n\r\n{}",
            version_to_string(&self.version),
            status_to_string(&self.status),
            self.payload.len(),
            self.payload
        };

        r
    }
}

