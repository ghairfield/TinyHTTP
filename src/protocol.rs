//! HTTP Protocols
//!
//! Greg Hairfield
//! CS410P Rust Programming
//! Spring 2021

/// HTTP request method.
///     Get: Standard request, supply the resource requested
///     Head: Same as GET but do not send content
///     Post: Client is responding with information
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
    Unknown,
}

/// HTTP request version. It comes as either a simple request, HTTP/1.0 or
/// HTTP/1.1. Anything else is invalid.
///
/// Currently TinyHTTP communicates according to the HTTP/1.0 specification.
/// The biggest upgrade to HTTP/1.1 is HTTPS and additional header fields.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RequestVersion {
    SimpleRequest, // Essentially HTTP/0.9
    HTTP1,         // HTTP/1.0
    HTTP11,        // HTTP/1.1
    Unknown,
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
pub enum RequestField {
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
    Unknown,
}

/// Response codes for HTML/1.0
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    Unknown,
}

/// Get the string representation of a request type.
pub fn method_to_string(r: &RequestMethod) -> String {
    match r {
        RequestMethod::Get => "GET".to_string(),
        RequestMethod::Head => "HEAD".to_string(),
        RequestMethod::Post => "POST".to_string(),
        RequestMethod::Put => "PUT".to_string(),
        RequestMethod::Link => "LINK".to_string(),
        RequestMethod::Unlink => "UNLINK".to_string(),
        RequestMethod::Delete => "DELETE".to_string(),
        RequestMethod::Unknown => "Unknown".to_string(),
    }
}

pub fn status_to_string(s: &StatusCode) -> String {
    match s {
        StatusCode::OK => "200 OK".to_string(),
        StatusCode::Created => "201 Created".to_string(),
        StatusCode::Accepted => "202 Accepted".to_string(),
        StatusCode::NoContent => "204 No Content".to_string(),
        StatusCode::MovedPermanently => "301 Moved Permanently".to_string(),
        StatusCode::MovedTemporarily => "202 Moved Temporarily".to_string(),
        StatusCode::NotModified => "304 Not Modified".to_string(),
        StatusCode::BadRequest => "400 Bad Request".to_string(),
        StatusCode::Unauthorized => "401 Unauthorized".to_string(),
        StatusCode::Forbidden => "403 Forbidden".to_string(),
        StatusCode::NotFound => "404 Not Found".to_string(),
        StatusCode::InternalServerError => "500 Internal Server Error".to_string(),
        StatusCode::NotImplemented => "501 Not Implemented".to_string(),
        StatusCode::BadGateway => "502 Bad Gateway".to_string(),
        StatusCode::ServiceUnavailable => "503 Service Unavailable".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Get the string representation of a HTTP version
pub fn version_to_string(r: &RequestVersion) -> String {
    match r {
        RequestVersion::SimpleRequest => "Simple Request".to_string(),
        RequestVersion::HTTP1 => "HTTP/1.0".to_string(),
        RequestVersion::HTTP11 => "HTTP/1.1".to_string(),
        RequestVersion::Unknown => "Unknown".to_string(),
    }
}

/// Get the HTTP field type as a string
pub fn field_to_string(r: &RequestField) -> String {
    match r {
        RequestField::Allow => "Allow: ".to_string(),
        RequestField::Authorization => "Authorization: ".to_string(),
        RequestField::ContentEncoding => "Content-Encoding: ".to_string(),
        RequestField::ContentLength => "Content-Length: ".to_string(),
        RequestField::ContentType => "Content-Type: ".to_string(),
        RequestField::Date => "Date: ".to_string(),
        RequestField::Expires => "Expires: ".to_string(),
        RequestField::FromField => "From: ".to_string(),
        RequestField::IfModifiedSince => "If-Modified-Since: ".to_string(),
        RequestField::LastModified => "Last-Modified: ".to_string(),
        RequestField::Location => "Location: ".to_string(),
        RequestField::Pragma => "Pragma: ".to_string(),
        RequestField::Referer => "Refer: ".to_string(),
        RequestField::Server => "Server: ".to_string(),
        RequestField::UserAgent => "User-Agent: ".to_string(),
        RequestField::WwwAuthenticate => "WWW-Authenticate: ".to_string(),
        RequestField::Unknown => "Unknown: ".to_string(),
    }
}
