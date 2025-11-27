use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);

            // Parse the request line
            let request_line = request.lines().next().unwrap_or("");

            println!("Request: {}", request_line);

            // Route the request
            let (status_line, content) = if request_line.starts_with("GET / ") {
                ("HTTP/1.1 200 OK", get_home_page())
            } else if request_line.starts_with("GET /about ") {
                ("HTTP/1.1 200 OK", get_about_page())
            } else if request_line.starts_with("GET /api/hello ") {
                ("HTTP/1.1 200 OK", get_json_response())
            } else {
                ("HTTP/1.1 404 NOT FOUND", get_404_page())
            };

            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                content.len(),
                content
            );

            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Failed to send response: {}", e);
            }

            if let Err(e) = stream.flush() {
                eprintln!("Failed to flush stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}

fn get_home_page() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust HTTP Server</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1 {
            color: #333;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        a {
            color: #007bff;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
        ul {
            line-height: 2;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Welcome to Rust HTTP Server</h1>
        <p>This is a simple HTTP server built with Rust using only the standard library!</p>
        <h2>Features:</h2>
        <ul>
            <li>No external dependencies</li>
            <li>Multi-threaded connection handling</li>
            <li>Basic routing</li>
            <li>Static HTML responses</li>
        </ul>
        <h2>Try these routes:</h2>
        <ul>
            <li><a href="/">Home</a> (you are here)</li>
            <li><a href="/about">About</a></li>
            <li><a href="/api/hello">API Endpoint (JSON)</a></li>
        </ul>
    </div>
</body>
</html>"#
    )
}

fn get_about_page() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>About - Rust HTTP Server</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1 {
            color: #333;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        a {
            color: #007bff;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>About This Server</h1>
        <p>This HTTP server is written in Rust using only the standard library.</p>
        <p>It demonstrates:</p>
        <ul>
            <li>TCP socket programming with <code>std::net::TcpListener</code></li>
            <li>Multi-threading with <code>std::thread</code></li>
            <li>HTTP protocol basics</li>
            <li>Request parsing and routing</li>
        </ul>
        <p><a href="/">← Back to Home</a></p>
    </div>
</body>
</html>"#
    )
}

fn get_json_response() -> String {
    String::from(r#"{"message": "Hello from Rust!", "status": "success", "server": "rust-stdlib-http"}"#)
}

fn get_404_page() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 Not Found</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1 {
            color: #d9534f;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        a {
            color: #007bff;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>404 - Page Not Found</h1>
        <p>The page you're looking for doesn't exist.</p>
        <p><a href="/">← Back to Home</a></p>
    </div>
</body>
</html>"#
    )
}
