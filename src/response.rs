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

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use crate::protocol::*;
use crate::request;

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
            payload: String::new(),
        }
    }

    /// Build a response for the client. 
    pub fn create_response(&mut self, h: &request::Header) {
        if !h.is_valid() {
            self.status = StatusCode::BadRequest;
            return
        }

        match self.get_resource(h.get_path()) {
            Ok(_) => self.status = StatusCode::OK,
            Err(_) => {
                self.status = StatusCode::NotFound;
                self.payload = "".to_string();
                return
            },
        };

        self.status = StatusCode::OK;
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
        let r;
        if self.status == StatusCode::BadRequest    ||
           self.status == StatusCode::Unauthorized  ||
           self.status == StatusCode::Forbidden     ||
           self.status == StatusCode::NotFound 
        {
            r = format!{
            "{} {}\r\n\r\n",
            version_to_string(&self.version),
            status_to_string(&self.status),
            };
        } else {
            r = format!{
                "{} {}\r\nContent-Lenght: {}\r\n\r\n{}",
                version_to_string(&self.version),
                status_to_string(&self.status),
                self.payload.len(),
                self.payload
            };
        }

        r
    }

    fn get_resource(&mut self, p: &str) -> Result<(), ResponseError> {
        if p == "/" {
            let mut file = match File::open("http/index.html") {
                Ok(file) => file,
                Err(x) => return Err(ResponseError {
                    message: format!("Could not open file! {}", x),
                    line: line!(),
                    column: column!(),
                }),
            };

            match file.read_to_string(&mut self.payload) {
                Ok(_) => return Ok(()),
                Err(x) => return Err(ResponseError {
                    message: format!("Could not open file! {}", x),
                    line: line!(),
                    column: column!(),
                }),
            };
        }
        /*
         * else
         */
        Err(ResponseError {
            message: "Could not find file!".to_string(),
            line: line!(),
            column: column!(),
        })
    }
}

