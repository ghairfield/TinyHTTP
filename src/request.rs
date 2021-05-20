//! HTTP Request Handler
//!
//! HTTP request module. Parses raw byte reads from the TCP stream
//! and can answer questions about the request. Please see RFC-1945
//! for more information.
//!
//! Greg Hairfield
//! CS410P Rust Programming
//! Spring 2021
//!

use std::str;
use std::collections::HashMap;

use crate::protocol; 

/// The standard error that the request parser will produce if there
/// is any problem parsing the request. For the most part, if the 
/// request line is bad, then the entire request is bad. A basic
/// request *must* contain: GET /CLRF
/// This is the simple request, other fields and definitions are 
/// optional and augment the request.
#[derive(Debug, Clone)]
pub struct ParsingError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

/// When a request is initiated, the contents of 
/// that request are stored here. 
///
/// If `valid` is false, then there is no guarantee 
/// what else is valid in the header. Some, all or 
/// none of the fields may be completed, but they 
/// are guaranteed to be initialized. 
///
/// Also if `request_version` is RequestVersion::SimpleRequest
/// then the only attributes here that will of been implemented
/// are `method`, `version` and `path`. 
/// A request of this type would be a HTTP/0.9 request.
#[derive(Debug)]
pub struct Header {
    valid: bool,
    method: protocol::RequestMethod,
    version: protocol::RequestVersion,
    path: String,
    fields: HashMap<protocol::RequestField, String>,
    unknown_fields: HashMap<String, String>,
}

impl Header {
    pub fn new(buf: &[u8]) -> Result<Header, ParsingError> {
        let request = str::from_utf8(&buf).unwrap().to_string();
        let mut header = init_header();

        // A simple request is defined as
        //      |GET /CRLF|
        // Anything less than this is not a valid request
        if request.len() < 7 {
            return Err(ParsingError {
                message: "Request is not valid.".to_string(),
                line: line!(),
                column: column!(),
            });
        }

        let mut parts: Vec<&str> = request.split("\r\n").collect();

        let method: Vec<&str> = parts[0].split(' ').collect();
        if method.len() < 2 {
            return Err(ParsingError {
                message: "Invalid Request".to_string(),
                line: line!(),
                column: column!()
            });
        }

        match method[0] {
            "GET" => header.method = protocol::RequestMethod::Get,
            "HEAD" => header.method = protocol::RequestMethod::Head,
            "POST" => header.method = protocol::RequestMethod::Post,
            "PUT" => header.method = protocol::RequestMethod::Put,
            "LINK" => header.method = protocol::RequestMethod::Link,
            "UNLINK" => header.method = protocol::RequestMethod::Unlink,
            "DELETE" => header.method = protocol::RequestMethod::Delete,
            _ => {
                return Err(ParsingError {
                    message: "Could not find method type".to_string(),
                    line: line!(),
                    column: column!()
                });
            }
        }

        header.path = method[1].to_string();

        if method.len() == 2 {
            header.version = protocol::RequestVersion::SimpleRequest;
        }
        else if method[2] == "HTTP/1.0" {
            header.version = protocol::RequestVersion::HTTP1;
        }
        else if method[2] == "HTTP/1.1" {
            header.version = protocol::RequestVersion::HTTP11;
        }
        else {
            return Err(ParsingError {
                message: "Invalid HTTP version.".to_string(),
                line: line!(),
                column: column!(),
            });
        }

        // Remove the method line since we parsed it already
        parts.remove(0);
        header.parse_fields(&parts); 

        // If we get here the request is valid
        header.valid = true;
        Ok(header)
    }

    /// Print the contents of the header field
    /// in plain text to stdout. Used for development
    pub fn print(&self) {
        let method = protocol::method_to_string(self.method);
        let version = protocol::version_to_string(self.version);

        println!("Request Line: {}, Path: {}, Version {}",
            method, self.path, version);

        println!("---- Known Fields ----");
        for (key, value) in &self.fields {
            let k = protocol::field_to_string(*key);
            println!("Field: {} -- Value: {}", k, value);
        }
        println!("---- Unknown Fields ----");
        for (key, value) in &self.unknown_fields {
            println!("Field: {} -- Value: {}", key, value);
        }
    }

    // According to RFC1945 any unrecognized header fields are to 
    // be treated as `Entity-Header` fields. Also the spec allows 
    // for experimental headers as long as both parties in 
    // communication recognize them. 
    //
    // What ever the field is, we store it. Unknown fields are 
    // stored separately than known fields. 
    fn parse_fields(&mut self, parts: &Vec<&str>) {
        for i in parts {
            let x: Vec<&str> = i.split(": ").collect();
           
            if x.len() == 1 {
                // TODO: Is there a better way to deal with this??
                // We could be here for 2 reasons:
                //   1. The final field in a request is CLRF
                //   2. An invalid `field: parameter` with no parameter 
                // Either way, we ignore it.
                continue;
            }

            let field = Header::field_to_type(x[0]);
            if field == protocol::RequestField::Unknown {
                self.unknown_fields.insert(
                    x[0].to_string(),
                    x[1].to_string()
                );
            }
            else {
                self.fields.insert(
                    field,
                    x[1].to_string()
                );
            }
        }
    }

    // Convert a request field to a known type. 
    fn field_to_type(f: &str) -> protocol::RequestField {
        match f {
            "Allow" => return protocol::RequestField::Allow,
            "Authorization" => return protocol::RequestField::Authorization,
            "Content-Encoding" => return protocol::RequestField::ContentEncoding,
            "Content-Length" => return protocol::RequestField::ContentLength,
            "Content-Type" => return protocol::RequestField::ContentType,
            "Date" => return protocol::RequestField::Date,
            "Expires" => return protocol::RequestField::Expires,
            "From" => return protocol::RequestField::FromField,
            "If-Modified-Since" => return protocol::RequestField::IfModifiedSince,
            "Last-Modified" => return protocol::RequestField::LastModified,
            "Location" => return protocol::RequestField::Location,
            "Pragma" => return protocol::RequestField::Pragma,
            "Referer" => return protocol::RequestField::Referer,
            "Server" => return protocol::RequestField::Server,
            "User-Agent" => return protocol::RequestField::UserAgent,
            "WWW-Authenticate" => return protocol::RequestField::WwwAuthenticate,
            _ => return protocol::RequestField::Unknown,
        }
    }
} // impl Header

// Create a empty header
fn init_header() -> Header {
    let header = Header {
        valid: false,
        method: protocol::RequestMethod::Unknown,
        version: protocol::RequestVersion::Unknown,
        path: String::new(),
        fields: HashMap::new(),
        unknown_fields: HashMap::new(),
    };

    header
}
