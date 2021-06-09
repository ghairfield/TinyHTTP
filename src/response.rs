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

use chrono::{DateTime, Local, TimeZone, Utc};
use std::collections::HashMap;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

use crate::configuration::CONFIG;
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
    pub content: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: StatusCode::Unknown,
            version: RequestVersion::HTTP1,
            fields: HashMap::<String, String>::new(),
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

    /// Format HTTP response to network ready data.
    pub fn respond(&mut self) -> Vec<u8> {
        let mut resp_header;
        let mut r;

        // If there is a bad request, just respond with error.
        if self.status == StatusCode::BadRequest
            || self.status == StatusCode::Unauthorized
            || self.status == StatusCode::Forbidden
            || self.status == StatusCode::NotFound
        {
            r = format! {
                "{} {}\r\n\r\n",
                version_to_string(&self.version),
                status_to_string(&self.status)
            };

            resp_header = r.as_bytes().to_vec();
        } else {
            // We have a request that needs a response
            r = format!(
                "{} {}\r\n",
                version_to_string(&self.version),
                status_to_string(&self.status)
            );

            for (key, value) in &self.fields {
                r.push_str(&format!("{}{}\r\n", key, value));
            }

            r.push_str("\r\n");

            resp_header = r.as_bytes().to_vec();
            if !self.content.is_empty() {
                resp_header.append(&mut self.content);
            }
        }

        resp_header
    }

    fn get_last_modified(meta: &Metadata) -> Result<String, ResponseError> {
        let lm = match meta.modified() {
            Ok(lm) => lm,
            Err(_) => {
                return Err(ResponseError {
                    message: "Could not get modifed data on file".to_string(),
                    line: line!(),
                    column: column!(),
                })
            }
        };

        let sec_from_epoch = lm.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let local_dt = Local.timestamp(sec_from_epoch.as_secs() as i64, 0);
        let utc_dt: DateTime<Utc> = DateTime::from(local_dt);

        Ok(format!("{}", utc_dt.format("%a, %d %b %Y %H:%M:%S GMT")))
    }

    fn get_resource(&mut self, p: &str) -> Result<(), ResponseError> {
        let doc_root = &CONFIG.doc_root;
        let index = match &CONFIG.root_file {
            Some(x) => &x,
            None => match &CONFIG.default_root_file {
                Some(x) => x,
                _ => {
                    return Err(ResponseError {
                        message: "Could not find a default root file!".to_string(),
                        line: line!(),
                        column: column!(),
                    })
                }
            },
        };

        if p == "/" {
            // Get the home page, specified by Config.toml -> doc_root/default_doc_root
            let mut file = match File::open(&format!("{}/{}", doc_root, index)) {
                Ok(file) => file,
                Err(x) => {
                    return Err(ResponseError {
                        message: format!("Could not open file! {}", x),
                        line: line!(),
                        column: column!(),
                    })
                }
            };

            let meta = match file.metadata() {
                Ok(meta) => meta,
                Err(_) => {
                    return Err(ResponseError {
                        message: "Could not get meta data on file".to_string(),
                        line: line!(),
                        column: column!(),
                    })
                }
            };

            if let Ok(time) = Response::get_last_modified(&meta) {
                self.fields
                    .insert(field_to_string(&RequestField::LastModified), time);
            }

            match file.read_to_end(&mut self.content) {
                Ok(size) => {
                    self.fields.insert(
                        field_to_string(&RequestField::ContentLength),
                        size.to_string(),
                    );
                }
                Err(x) => {
                    return Err(ResponseError {
                        message: format!("Could not read file! {}", x),
                        line: line!(),
                        column: column!(),
                    })
                }
            };
        } else {
            // If the resource is not the index, we want to walk the directory
            // tree and find it.
            let p = format!("{}{}", CONFIG.doc_root, p);
            let path = Path::new(&p);

            println!("Path: {:?}", p);

            if path.exists() {
                let mut file = match File::open(path) {
                    Ok(file) => file,
                    Err(x) => {
                        return Err(ResponseError {
                            message: format!("Could not open file! {}", x),
                            line: line!(),
                            column: column!(),
                        })
                    }
                };

                let meta = match file.metadata() {
                    Ok(meta) => meta,
                    Err(_) => {
                        return Err(ResponseError {
                            message: "Could not get meta data on file".to_string(),
                            line: line!(),
                            column: column!(),
                        })
                    }
                };

                if let Ok(time) = Response::get_last_modified(&meta) {
                    self.fields
                        .insert(field_to_string(&RequestField::LastModified), time);
                }

                match file.read_to_end(&mut self.content) {
                    Ok(size) => {
                        self.fields.insert(
                            field_to_string(&RequestField::ContentLength),
                            size.to_string(),
                        );
                    }
                    Err(x) => {
                        return Err(ResponseError {
                            message: format!("Could not read file! {}", x),
                            line: line!(),
                            column: column!(),
                        })
                    }
                }
            }
        }

        Ok(())
    }

    // Handle a GET request from a client
    fn get_request(&mut self, req: &request::Header) {
        match self.get_resource(req.get_path()) {
            Ok(_) => self.status = StatusCode::OK,
            Err(_) => {
                self.status = StatusCode::NotFound;
            }
        }

        // Are their any fields we want to look at?
    }

    fn head_request(&mut self, req: &request::Header) {
        match self.get_resource(req.get_path()) {
            Ok(_) => self.status = StatusCode::OK,
            Err(_) => {
                self.status = StatusCode::NotFound;
            }
        }
        self.content.clear();
    }

    fn post_request(&mut self, req: &request::Header) {
        todo!()    
    }

    fn unsupported_request(&mut self, req: &request::Header) {
        todo!()
    }
}
