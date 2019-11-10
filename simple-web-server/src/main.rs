use std::net::TcpListener;
use std::io::{self};

fn main() {

    let port= 80;
    let port_str = port.to_string();
    if !check_port_availability(port) {
        let stdout = io::stdout();
        let _handle = stdout.lock();

        // @todo - Panic or just print and exit with err 1? Sample code below
        panic!("Unable to bind to port 80!");

//        println!("Unable to bind to port 80!");
//        process.exit(0x0100);
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_str)).unwrap();
    println!("Listener started on port 80, accepting connections...");

    for _stream in listener.incoming() {
        println!("Received a request on port 80!\r\n");
    }
}

fn check_port_availability(port: u16) -> bool {
    return match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false
    };
}