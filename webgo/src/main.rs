use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn handleClient(mut connStream:TcpStream) {
    let msg = "dddd";
    loop{
        connStream.write(msg.as_bytes());
        std::thread::sleep_ms(10);
    }
   
}

fn startListner(ip: &str, port: i32) {
    let addr = format!("{0}:{1}", ip, port);
    println!("addr:{}", addr);

    let h = std::thread::spawn(|| {
        let listener = TcpListener::bind(addr).unwrap();

        loop {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut expr) => {
                        handleClient(expr);
                    }
                    Err(e) => {
                        println!("hello world....{:#?}", e);
                    }
                }
            }
        }
    });
    h.join();
}

fn main() {
    let port = 9000;
    let ip = "127.0.0.1";

    startListner(ip, port);
}
