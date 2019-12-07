
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:20080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {handle_client(stream);});
            }
            Err(e) => { println!("{}", e); }
        }
    }
  
}

fn handle_client(mut stream:&TcpStream) {
    println!("request in");
    stream.write(b"welcome\r\n").unwrap();
}