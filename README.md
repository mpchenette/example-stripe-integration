# Rust HTTP Server (stdlib only)

A simple HTTP server written in Rust using only the standard library - no external dependencies!

## Features

- **Zero dependencies**: Built using only `std` library
- **Multi-threaded**: Each connection is handled in a separate thread
- **Basic routing**: Supports multiple endpoints
- **HTTP/1.1 compliant**: Proper response formatting

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

The server will start on `http://127.0.0.1:7878`

## Available Routes

- **GET /** - Home page with server information
- **GET /about** - About page with implementation details
- **GET /api/hello** - JSON API endpoint
- **Any other route** - Returns 404 page

## Testing

Once the server is running, you can test it with:

```bash
# Using curl
curl http://127.0.0.1:7878/
curl http://127.0.0.1:7878/about
curl http://127.0.0.1:7878/api/hello

# Or open in your browser
open http://127.0.0.1:7878
```

## Implementation Details

The server uses:
- `std::net::TcpListener` for accepting TCP connections
- `std::thread` for handling concurrent requests
- `std::io::{Read, Write}` for reading requests and writing responses
- Manual HTTP parsing and response formatting

## Architecture

1. **Main loop**: Binds to port 7878 and listens for incoming connections
2. **Connection handling**: Each connection spawns a new thread
3. **Request parsing**: Reads the request line to determine the route
4. **Response generation**: Returns appropriate HTML or JSON content
5. **HTTP formatting**: Builds proper HTTP/1.1 responses with status and headers
