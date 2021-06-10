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

use std::collections::HashMap;
use std::str;

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
    /// Is request in valid format?
    valid: bool,
    /// Request type e.g GET, HEAD, POST
    method: protocol::RequestMethod,
    /// Request version e.g SimpleRequest, HTTP/1.0, HTTP/1.1
    version: protocol::RequestVersion,
    /// URI
    path: String,
    /// HTTP/1.0 Known fields
    fields: HashMap<protocol::RequestField, String>,
    /// Possible fields from HTTP/1.1 request, non-documented fileds.
    /// See [RFC 1945, Section 10. Header Field Definitions]
    unknown_fields: HashMap<String, String>,
    /// POST fields. It should only be used with a POST request
    /// See [RFC 1945 Secion 8.3 POST]
    post_fields: HashMap<String, String>,
}

// Create a empty header
impl Default for Header {
    fn default() -> Self {
        Header {
            valid: false,
            method: protocol::RequestMethod::Unknown,
            version: protocol::RequestVersion::Unknown,
            path: String::new(),
            fields: HashMap::new(),
            unknown_fields: HashMap::new(),
            post_fields: HashMap::new(),
        }
    }
}

impl Header {
    pub fn new(buf: &[u8]) -> Self {
        let request = str::from_utf8(&buf).unwrap().to_string();
        let mut header = Header::default();

        // A simple request is defined as
        //      |GET /CRLF|
        // Anything less than this is not a valid request
        if request.len() < 7 {
            return header;
        }

        let mut parts: Vec<&str> = request.split("\r\n").collect();
        let method: Vec<&str> = parts[0].split(' ').collect();
        if method.len() < 2 {
            return header;
        }

        match method[0] {
            "GET" => header.method = protocol::RequestMethod::Get,
            "HEAD" => header.method = protocol::RequestMethod::Head,
            "POST" => header.method = protocol::RequestMethod::Post,
            "PUT" => header.method = protocol::RequestMethod::Put,
            "LINK" => header.method = protocol::RequestMethod::Link,
            "UNLINK" => header.method = protocol::RequestMethod::Unlink,
            "DELETE" => header.method = protocol::RequestMethod::Delete,
            _ => return header,
        }

        header.path = method[1].to_string();

        if method.len() == 2 {
            header.version = protocol::RequestVersion::SimpleRequest;
        } else if method[2] == "HTTP/1.0" {
            header.version = protocol::RequestVersion::HTTP1;
        } else if method[2] == "HTTP/1.1" {
            header.version = protocol::RequestVersion::HTTP11;
        } else {
            return header;
        }

        // Remove the method line since we parsed it already
        parts.remove(0);
        header.parse_fields(&parts);

        // If we get here the request is valid
        header.valid = true;
        header
    }

    /// Print the contents of the header field
    /// in plain text to stdout. Used for development
    pub fn print(&self) {
        let method = protocol::method_to_string(&self.method);
        let version = protocol::version_to_string(&self.version);

        println!(
            "Request Line: {}, Path: {}, Version {}",
            method, self.path, version
        );

        println!("---- Known Fields ----");
        for (key, value) in &self.fields {
            let k = protocol::field_to_string(key);
            println!("Field: {} -- Value: {}", k, value);
        }
        println!("---- Unknown Fields ----");
        for (key, value) in &self.unknown_fields {
            println!("Field: {} -- Value: {}", key, value);
        }
        if self.method == protocol::RequestMethod::Post {
            println!("---- POST Fields ----");
            for (key, value) in &self.post_fields {
                println!("Name: {} -- Value: {}", key, value);
            }
        }
    }

    /// Get the validity of the request. If this returns false, then all
    /// other header fields *might be* invalid.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Get the path of the request
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Get the method of the request
    pub fn get_method(&self) -> protocol::RequestMethod {
        self.method
    }

    /*
     * I'm not sure what the best way to deal with this is.
     *
     * Really this function is a `todo!()` function since right now TinyHTTP
     * doesn't look at the request header fields.
     *
     * In the future this is planned to be used.
     *
     * Get a request header field's value.
    #[allow(dead_code)]
    pub fn get_header_field(&self, r: protocol::RequestField) -> Option<&str> {
        self.fields.get(&r).map(|x| &x[..])
    }
    */

    // According to RFC1945 any unrecognized header fields are to
    // be treated as `Entity-Header` fields. Also the spec allows
    // for experimental headers as long as both parties in
    // communication recognize them.
    //
    // What ever the field is, we store it. Unknown fields are
    // stored separately than known fields.
    fn parse_fields(&mut self, parts: &[&str]) {
        for i in parts {
            let x: Vec<&str> = i.split(": ").collect();

            if x.len() == 1 {
                // Is there a better way to deal with this??
                // We could be here for 2 reasons:
                //   1: This is a POST request with fields, which we capture.
                //   2. An invalid `field: parameter` with no parameter (invalid request)
                let field: Vec<&str> = x[0].split("&").collect();
                if !field.is_empty() {
                    for i in &field {
                        let j: Vec<&str> = i.split("=").collect();
                        if j.len() == 2 {
                            // POST field!
                            self.post_fields.insert(j[0].to_string(), j[1].to_string());
                        }
                    }
                }
                continue;
            }

            let field = Header::field_to_type(x[0]);
            if field == protocol::RequestField::Unknown {
                self.unknown_fields
                    .insert(x[0].to_string(), x[1].to_string());
            } else {
                self.fields.insert(field, x[1].to_string());
            }
        }
    }

    // Convert a request field to a known type.
    fn field_to_type(f: &str) -> protocol::RequestField {
        match f {
            "Allow" => protocol::RequestField::Allow,
            "Authorization" => protocol::RequestField::Authorization,
            "Content-Encoding" => protocol::RequestField::ContentEncoding,
            "Content-Length" => protocol::RequestField::ContentLength,
            "Content-Type" => protocol::RequestField::ContentType,
            "Date" => protocol::RequestField::Date,
            "Expires" => protocol::RequestField::Expires,
            "From" => protocol::RequestField::FromField,
            "If-Modified-Since" => protocol::RequestField::IfModifiedSince,
            "Last-Modified" => protocol::RequestField::LastModified,
            "Location" => protocol::RequestField::Location,
            "Pragma" => protocol::RequestField::Pragma,
            "Referer" => protocol::RequestField::Referer,
            "Server" => protocol::RequestField::Server,
            "User-Agent" => protocol::RequestField::UserAgent,
            "WWW-Authenticate" => protocol::RequestField::WwwAuthenticate,
            _ => protocol::RequestField::Unknown,
        }
    }
} // impl Header
