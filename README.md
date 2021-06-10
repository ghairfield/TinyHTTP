# TinyHTTP

CS410P Spring 2021 Tiny Rust HTTP/1.0 Server in Rust.

Greg Hairfield - CS410P - Spring 2021

A quasi [HTTP/1.0](https://www.ietf.org/rfc/rfc1945.txt) server written in
Rust. 

# Description

Originally this was to be a HTTP/0.9 complient server and moved on to create
a HTTP/1.0 quasi complient server. A HTTP/0.9 server doesn't really 
have much to it other than a HTML file server and I was looking for more of
a challenge. Making the HTTP/1.0 server might of been more than I could chew
at the end of the quarter. While the server does work with `GET`, `POST` and
`HEAD` request (which is all HTTP/1.0 requires) I didn't extend any of the
other request methods. 

This project is obviously a toy and should never be used in the real world.
It is meant to be a fun project to learn Rust and get a better understanding
of HTTP and how it works. 

# Usage

See the `examples` folder for usage. All configuration is done by the 
`Config.toml` file, read the description of options. Basically one needs to 
`use tiny_http;` and call `tiny_http::tiny_http()` to run the server.

The included `http` folder is for example use. 

# TODO

### HTTP/0.9

- [X] Accept a connection
- [X] Respond to a request
- [X] Close the connection
- [X] Method Definitions
  - [X] GET
- [X] Fetches resource 

### HTTP/1.0

- [X] Accept a connection
- [X] Respond to a request
- [X] Close the connection (each time, ignores `Keep-alive`)
- [X] Method Definitions
  - [X] GET
  - [X] HEAD
  - [X] POST
- [ ] Additional Request Methods (extended HTTP/1.0)
  - [ ] PUT
  - [ ] DELETE
  - [ ] LINK
  - [ ] UNLINK
- [ ] Status Codes
  - [ ] Informational 1xx
  - [X] Successful 2xx
  - [ ] Redirection 3xx
  - [X] Client Error 4xx
  - [X] Server Error 5xx
- [ ] Header Fields
  - [ ] Allow
  - [ ] Authorization
  - [ ] Content-Encoding
  - [X] Content-Length
  - [ ] Content-Type
  - [ ] Date
  - [ ] Expires
  - [ ] From
  - [ ] If-Modified-Since
  - [X] Last-Modified
  - [ ] Location
  - [ ] Pragma
  - [X] Referer
  - [ ] Server
  - [X] User-Agent (recorded)
  - [ ] WWW-Authenticate
- [ ] Additional Header Field Definitions (extended HTTP/1.0)
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

### What worked

Working with TCP in Rust is supprising easy on a basic level. There were a few
times I refactored the code which made the project more readable, but I think
overall the quality of programming is low. Given more time this wouldn't be an
issue.

### What didn't work (or how I could of planned better)

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

### What I wish I could of implemented

As I said above, a cache would of been a great improvment especially with as 
few files that are currently included. I would of also liked to have better 
thread control, such as implementing `Keep-Alive`.  Another one I would of
liked to implement is `If-Modified-Since` as that would ease server load.

Many of the fields from the client are recorded in a `HashMap` so they are 
available to the `response` but their functionality is missing. For example 
`Referer` and `User-Agent` are just informal.

I also wanted to implement a thread pool for the project. Currently it spawns
a thread per request. Being a toy implementation is is ok, but for any serious
project a thread pool would be needed. 

# License

This project is licensed under the [MIT license](LICENSE). A copy should be
supplied with the code. 
