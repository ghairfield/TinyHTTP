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
use std::path::Path;
use std::ffi::OsStr;

use std::collections::HashMap;
use crate::protocol::*;
use crate::request;
use crate::configuration::CONFIG;

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
    pub content: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status:  StatusCode::Unknown,
            version: RequestVersion::HTTP1,
            fields:  HashMap::<String, String>::new(),
            content: Vec::<u8>::new(),
        }
    }
}

impl Response {
    /// Create a new Response structure with some default values. 
    pub fn new(h: &request::Header) -> Self {
        let mut response = Response::default();
        
        if !h.is_valid() {
            response.status = StatusCode::BadRequest;
            return response;
        }

        // Respond to the type of method
        let m = h.get_method();
        match m {
            RequestMethod::Get => response.get_request(&h),
            RequestMethod::Head => response.head_request(&h),
            RequestMethod::Post => response.post_request(&h),
            _ => response.unsupported_request(&h),
        }
        
        response
    }

    pub fn to_network(&mut self) -> Vec<u8> {
        let mut resp_header = Vec::<u8>::new();
        /*
        r.push(version_to_string(self.version).as_bytes().to_vec());
        r.push(b' ');
        r.push(status_to_string(self.status).as_bytes().to_vec());
        r.push(b"\r\n");
        r.push(b"Content-Length: ");
        r.push(self.content.len().to_string().as_bytes().to_vec());
        r.push(b"\r\n\r\n");
        r.push(self.content.clone());
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
                "{} {}\r\nContent-Lenght: {}\r\n\r\n",
                version_to_string(&self.version),
                status_to_string(&self.status),
                self.content.len()
            };
        }

        resp_header = r.as_bytes().to_vec();
        if !self.content.is_empty() {
            resp_header.append(&mut self.content);
        }

        resp_header
    }

    fn get_resource(&mut self, p: &str) -> Result<(), ResponseError> {
        if p == "/" {
            // Get the index
            let index = match &CONFIG.root_file {
                Some(x) => &x,
                None => {
                    match &CONFIG.default_root_file {
                        Some(x) => x,
                        _ => return Err(ResponseError {
                            message: "Could not find a default root file!".to_string(),
                            line: line!(),
                            column: column!(),
                        })
                    }
                }
            };
            let doc_root = &CONFIG.doc_root; 

            let mut file = match File::open(&format!("{}/{}", doc_root, index)) {
                Ok(file) => file,
                Err(x) => return Err(ResponseError {
                    message: format!("Could not open file! {}", x),
                    line: line!(),
                    column: column!(),
                }),
            };

            match file.read_to_end(&mut self.content) {
                Ok(_) => return Ok(()),
                Err(x) => return Err(ResponseError {
                    message: format!("Could not read file! {}", x),
                    line: line!(),
                    column: column!(),
                }),
            };
        } else {
            // If the resource is not the index, we want to walk the directory
            // tree and find it. 
            let p = format!("{}{}", CONFIG.doc_root, p);
            let path = Path::new(&p);
            if path.exists() {
                let mut file = match File::open(path) {
                    Ok(file) => file,
                    Err(x) => return Err(ResponseError {
                        message: format!("Could not open file! {}", x),
                        line: line!(),
                        column: column!(),
                    }),
                };
                match file.read_to_end(&mut self.content) {
                    Ok(_) => return Ok(()),
                    Err(x) => return Err(ResponseError {
                        message: format!("Could not read file! {}", x),
                        line: line!(),
                        column: column!(),
                    }),
                }
            }
        }

        Err(ResponseError {
            message: "Could not find file!".to_string(),
            line: line!(),
            column: column!(),
        })
    }
    
    // Handle a GET request from a client
    fn get_request(&mut self, req: &request::Header) {
        match self.get_resource(req.get_path()) {
            Ok(_) => self.status = StatusCode::OK,
            Err(_) => {
                self.status = StatusCode::NotFound;
            },
        }

        // Are their any fields we want to look at?
    }

    fn head_request(&mut self, req: &request::Header) {
        todo!()
    }

    fn post_request(&mut self, req: &request::Header) {
        todo!()
    }

    fn unsupported_request(&mut self, req: &request::Header) {
        todo!()
    }
}

