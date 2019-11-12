use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::{self};
use std::borrow::Cow;

fn main() {

    let port = 8080;
    let port_str = port.to_string();
    if !check_port_availability(port) {
        let stdout = io::stdout();
        let _handle = stdout.lock();

        // @todo - Panic or just print and exit with err 1? Sample code below
        panic!("Unable to bind to port {}!", port);

//        println!("Unable to bind to port 80!");
//        process.exit(0x0100);
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_str)).unwrap();
    println!("Listener started on port {}, accepting connections...", port);

    for stream in listener.incoming() {
        println!("Received a request on port {}!\r\n", port);
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn check_port_availability(port: u16) -> bool {
    return match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // Prints a string of the HTTP request to Terminal
    let http_str = String::from_utf8_lossy(&buffer[..]);
    println!("{}", http_str); // prints out http request TODO: remove this later

    let token = tokenize_http_req(http_str);
    println!("{}", token.method); // TODO: Remove this later


    let response = "HTTP/1.1 200 OK"; // Generic success response (placeholder)

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Basic version
// TODO: maybe add user_agent, content_length, connection in the future
struct HttpReq {
    method: String,
    // host: String,
    // content_type: String,
    // body: String,
}

// Early version, currently only looks for GET,POST,PUT,DELETE
fn tokenize_http_req(req: Cow<'_, str>) -> HttpReq {

    // splits string into newlines so it can be iterated over
    let header = req.lines();
    let mut method = String::new();

    for line in header {
        let words = line.split_whitespace();
        for word in words {
            if word == "GET" {
                method = word.to_string();
            }
            else if word == "POST" {
                method = word.to_string();
            }
            else if word == "PUT" {
                method = word.to_string();
            }
            else if word == "DELETE" {
                method = word.to_string();
            }
        }
    }

    let t = HttpReq {
        method: method,
    };

    return t
}