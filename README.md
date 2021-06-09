# TinyHTTP

CS410P Spring 2021 Tiny Rust HTTP/1.0 Server in Rust.

A quasi [HTTP/1.0](https://www.ietf.org/rfc/rfc1945.txt) server written in
Rust. 

# Description

Originally this was to be a HTTP/0.9 complient server. After implementing that,
I move on to upgrading to a HTTP/1.0 server. A HTTP/0.9 server doesn't really 
have much to it other than a HTML file server and I was looking for more of
a challenge. Making the HTTP/1.0 server might of been more than I could chew
at the end of the quarter. While the server does work with `GET`, `POST` and
`HEAD` request (which is all HTTP/1.0 requires) I didn't extend any of the
other request methods. 

# Usage

See the `examples` folder for usage.

# TODO

- [X] Accept a connection
- [X] Respond to a request
- [X] Close the connection (each time, ignores `Keep-alive`)
- [X] Method Definitions
  - [X] GET
  - [X] HEAD
  - [X] POST
- [ ] Additional Request Methods
  - [ ] PUT
  - [ ] DELETE
  - [ ] LINK
  - [ ] UNLINK
- [ ] Status Codes
  - [ ] Informational 1xx
  - [X] Successful 2xx
  - [ ] Redirection 3xx
  - [X] Client Error 4xx
  - [ ] Server Error 5xx
- [ ] Header Fields
  - [ ] Allow
  - [ ] Authorization
  - [ ] Content-Encoding
  - [ ] Content-Length
  - [ ] Content-Type
  - [ ] Date
  - [ ] Expires
  - [ ] From
  - [ ] If-Modified-Since
  - [ ] Last-Modified
  - [ ] Location
  - [ ] Pragma
  - [ ] Referer
  - [ ] Server
  - [X] User-Agent (recorded)
  - [ ] WWW-Authenticate
- [ ] Additional Header Field Definitions
  - [ ] Accept
  - [ ] Accept-Charset
  - [ ] Accept-Encoding
  - [ ] Accept-Language
  - [ ] Content-Language
  - [ ] Link
  - [ ] MIME-Version
  - [ ] Retry-After
  - [ ] Title
  - [ ] URI

# Learning

I started out working with file as strings using the `read_to_string` method.
Initially this is how I brought together all of the headers and content, a 
large string that I then coverted to `u8` by using `as_btyes`. It wasn't until
a few days before the assignment was due that I figured out it would not work
on photos and such. I ended up changing things to `u8` and is something I will
remember anytime I have to work with non-text files.

After our third assignment I realized I didn't know enough about lifetimes to
incorporate them into my project. One area I wish I could expand on would be
the file system. As it currently stands, each request TinyHTTP querys the file
system for the file and read it. Obviously this is poor design and I would of 
liked to incorporate a cache for the files. Maybe this is something I will 
tackle over the break.

One of Rust's infurating issues is time. I've never used a programming language
that how no convinent way to work with time as Rust. I find this dissapointing
especially since Rust sells itself as a systems language. Some of the issues 
were that time in HTTP/0.9 uses RFC1123 date and time where HTTP/1.1 uses 
RFC2282. 

Another place I think I went wrong is the error handling. At first I created an
error `struct` for each part (request, response, lib) but that was a mistake. 
In hindsight I think I would of created one for the entire crate. It made 
passing errors a little overly-robust. 
