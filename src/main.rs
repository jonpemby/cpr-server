use std::net::{TcpListener, TcpStream};
use std::io::Read;

const TEST_COMMAND: &str = "test";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3310")
        .expect("Failed to bind on port 3310");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_stream(s),
            Err(_) => handle_error(),
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = String::new();

    stream.read_to_string(&mut buf)
        .expect("Fatal error reading from stream");

    match buf.trim() {
        TEST_COMMAND => handle_test(),
        _ => {
            //
        }
    }
}

fn handle_error() {
    //
}

fn handle_test() {
    println!("bin/composer test");
}
