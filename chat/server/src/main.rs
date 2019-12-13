use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client_Write_Routine(mut stream:TcpStream) {
    println!("client  come in ip::{:#?}",stream.peer_addr());

    loop {
        stream.write(b"welcome\r\n");
        thread::sleep_ms(1000);
    }

   
}


fn doAcceptorService(acceptListener: TcpListener) {
    for stream in acceptListener.incoming() {
        match stream {
            Ok(stream) => {
                let handlerWrite = thread::spawn(move || {
                    handle_client_Write_Routine(stream);
                });

                println!("handler={:#?}",handlerWrite);
                handlerWrite.join();
              
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:20080").unwrap();
    doAcceptorService(listener);
}
