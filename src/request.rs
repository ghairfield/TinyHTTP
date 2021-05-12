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
//! # Examples
//!
//! ```
//! use request::Header;
//! ```

use std::str;
use std::collections::HashMap;

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

/// The method definitions TinyHTTP accepts and can generate
/// a response.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RequestMethod {
    // HTTP/1.0 methods
    Get,
    Head,
    Post,
    // extended HTTP/1.0 methods 
    Put,
    Delete,
    Link,
    Unlink,
    // exclusive HTTP/1.1 methods
    // others
    Unknown
}

/// HTTP request version. It comes as either a simple request, HTTP/1.0 or 
/// HTTP/1.1. Anything else is invalid. 
///
/// Currently TinyHTTP communicates according to the HTTP/1.0 specification. 
/// The biggest upgrade to HTTP/1.1 is HTTPS and additional header fields.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RequestVersion {
    SimpleRequest, // Essentially HTTP/0.9
    HTTP1,  // HTTP/1.0
    HTTP11, // HTTP/1.1
    Unknown
}

/// The types of header field definitions we accept. A simple explanation
/// of the fields follow. See [RFC 1945](https://www.ietf.org/rfc/rfc1945.txt)
/// for more information.
///     Accept: Lists the set of methods supported by the URI. Ignored of
///             part of a POST request
///     Authorization: A user agent that wishes to authenticate with the 
///             server. Mostly happens after a 401 response. 
///             Should be in the form: `"Authorization" : credentials`
///     Content-Encoding: Indicates what additional content coding has
///             been applied to the resource. 
///     Content-Length: The size of the `Entity-Body` less the header
///             information. The size of the payload. In the case of a
///             HEAD request, it represents "what would of been sent"
///     Content-Type: The media type of the `Entity-Body`. In the case of
///             HEAD request, it represents "what would of been sent"
///     Date: The date and time  the message originated. See
///             [RFC 1123](https://datatracker.ietf.org/doc/html/rfc1123)
///             [RFC 822](https://datatracker.ietf.org/doc/html/rfc822)
///             for valid date and time formats
///     Expires: The date/time when the entity is stale, or the resource
///             is no longer considered valid.
///     From: An e-mail address of the requesting user agent. Usually used
///             for robots and is meant for logging
///     If-Modified-Since: Used with GET method, if the requested resource
///             has not been modified since the time specified, return 304,
///             Not-Modified.
///     Last-Modified: The date and time at which the sender believes the 
///             resource was last modified.
///     Location: Identifies the exact location of a resource that was 
///             identified by the requested URI. For 3xx responses the
///             server must indicate the preferred URL
///     Pragma: Implementation specific directives that may apply to any
///             recipient along the request/response chain.
///     Referer: Allows the client to specify the URI of the resource from
///             which the request was obtained.
///     Server: Contains information about the software used by the origin
///             server to handle the request.
///     User-Agent: Request field containing information about the user
///             agent originating the request. 
///     WWW-Authenticate: Used with 401 Unauthorized response message. The
///             field consists of at least one challenge that indicates the
///             authentication scheme.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RequestHeaderField {
    /// HTTP/1.0
    Allow,
    Authorization,
    ContentEncoding,
    ContentLength,
    ContentType,
    Date,
    Expires,
    FromField,
    IfModifiedSince,
    LastModified,
    Location,
    Pragma,
    Referer,
    Server,
    UserAgent,
    WwwAuthenticate,
    // extended HTTP/1.0
    // exclusive HTTP/1.1
    // others
    Unknown
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
    method: RequestMethod,
    version: RequestVersion,
    path: String,
    fields: HashMap<RequestHeaderField, String>,
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
            "GET" => header.method = RequestMethod::Get,
            "HEAD" => header.method = RequestMethod::Head,
            "POST" => header.method = RequestMethod::Post,
            "PUT" => header.method = RequestMethod::Put,
            "LINK" => header.method = RequestMethod::Link,
            "UNLINK" => header.method = RequestMethod::Unlink,
            "DELETE" => header.method = RequestMethod::Delete,
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
            header.version = RequestVersion::SimpleRequest;
        }
        else if method[2] == "HTTP/1.0" {
            header.version = RequestVersion::HTTP1;
        }
        else if method[2] == "HTTP/1.1" {
            header.version = RequestVersion::HTTP11;
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
        let method = method_to_string(self.method);
        let version = version_to_string(self.version);

        println!("Request Line: {}, Path: {}, Version {}",
            method, self.path, version);

        println!("---- Known Fields ----");
        for (key, value) in &self.fields {
            let k = field_to_string(*key);
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
            if field == RequestHeaderField::Unknown {
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
    fn field_to_type(f: &str) -> RequestHeaderField {
        match f {
            "Allow" => return RequestHeaderField::Allow,
            "Authorization" => return RequestHeaderField::Authorization,
            "Content-Encoding" => return RequestHeaderField::ContentEncoding,
            "Content-Length" => return RequestHeaderField::ContentLength,
            "Content-Type" => return RequestHeaderField::ContentType,
            "Date" => return RequestHeaderField::Date,
            "Expires" => return RequestHeaderField::Expires,
            "From" => return RequestHeaderField::FromField,
            "If-Modified-Since" => return RequestHeaderField::IfModifiedSince,
            "Last-Modified" => return RequestHeaderField::LastModified,
            "Location" => return RequestHeaderField::Location,
            "Pragma" => return RequestHeaderField::Pragma,
            "Referer" => return RequestHeaderField::Referer,
            "Server" => return RequestHeaderField::Server,
            "User-Agent" => return RequestHeaderField::UserAgent,
            "WWW-Authenticate" => return RequestHeaderField::WwwAuthenticate,
            _ => return RequestHeaderField::Unknown,
        }
    }
} // impl Header

// Create a empty header
fn init_header() -> Header {
    let header = Header {
        valid: false,
        method: RequestMethod::Unknown,
        version: RequestVersion::Unknown,
        path: String::new(),
        fields: HashMap::new(),
        unknown_fields: HashMap::new(),
    };

    header
}

/// Get the string representation of a request type.
pub fn method_to_string(r: RequestMethod) -> String {
    let ret;

    match r {
        RequestMethod::Get => ret = "GET".to_string(), 
        RequestMethod::Head => ret = "HEAD".to_string(), 
        RequestMethod::Post => ret = "POST".to_string(), 
        RequestMethod::Put => ret = "PUT".to_string(),
        RequestMethod::Link => ret = "LINK".to_string(),
        RequestMethod::Unlink => ret = "UNLINK".to_string(),
        RequestMethod::Delete => ret = "DELETE".to_string(),
        RequestMethod::Unknown => ret = "Unknown".to_string(),
    }

    ret
}


/// Get the string representation of a HTTP version
pub fn version_to_string(r: RequestVersion) -> String {
    let ret;

    match r {
        RequestVersion::SimpleRequest => ret = "Simple Request".to_string(),
        RequestVersion::HTTP1 => ret = "HTTP/1.0".to_string(),
        RequestVersion::HTTP11 => ret = "HTTP/1.1".to_string(),
        RequestVersion::Unknown => ret = "Unknown".to_string(),
    }

    ret
}

/// Get the HTTP field type as a string
pub fn field_to_string(r: RequestHeaderField) -> String {
    let ret;

    match r {
        RequestHeaderField::Allow => ret = "Allow".to_string(),
        RequestHeaderField::Authorization => ret = "Authorization".to_string(),
        RequestHeaderField::ContentEncoding => ret = "Content-Encoding".to_string(),
        RequestHeaderField::ContentLength => ret = "Content-Length".to_string(),
        RequestHeaderField::ContentType => ret = "Content-Type".to_string(),
        RequestHeaderField::Date => ret = "Date".to_string(),
        RequestHeaderField::Expires => ret = "Expires".to_string(),
        RequestHeaderField::FromField => ret = "From".to_string(),
        RequestHeaderField::IfModifiedSince => ret = "If-Modified-Since".to_string(),
        RequestHeaderField::LastModified => ret = "Last-Modified".to_string(),
        RequestHeaderField::Location => ret = "Location".to_string(),
        RequestHeaderField::Pragma => ret = "Pragma".to_string(),
        RequestHeaderField::Referer => ret = "Refer".to_string(),
        RequestHeaderField::Server => ret = "Server".to_string(),
        RequestHeaderField::UserAgent => ret = "User-Agent".to_string(),
        RequestHeaderField::WwwAuthenticate => ret = "WWW-Authenticate".to_string(),
        RequestHeaderField::Unknown => ret = "Unknown".to_string(),
    }

    ret
}
