use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use std::io::ErrorKind;
 

fn createListener(ip: &str, port: i32) -> TcpListener {
    let addr = format!("{}:{}", ip, port);
    println!("addr:{}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    listener
}

const msg:[u8;3]=[65u8, 66u8, 67u8];
 
fn goWriteRoutine(conn: Arc<Mutex<TcpStream>>) {
    std::thread::spawn(move || {
        loop{
            let mut safeWriteStream = conn.lock().unwrap();
            
            match safeWriteStream.write(&msg)
            {
                 Ok(size)=>{

                 },

                 Err(e)=>{
                    match e.kind(){
                        ErrorKind::ConnectionAborted=>{
                                println!("client abort connection exit");
                                break ;
                        },
                        _=>{
                            println!("client abort connection exit e.kind():{:#?}",e.kind());
                               break ;
                        }

                    }
                 }

            }


            thread::sleep(Duration::from_millis(1000));
        };

    });
}

fn goReadRoutine(conn: Arc<Mutex<TcpStream>>) {
    // std::thread::spawn(|| {
    //     loop{
    //         let safeWriteStream = conn.lock().unwrap();
    //         safeWriteStream.write(&[65u8, 66u8, 67u8]);
    //         thread::sleep(Duration::from_millis(1000));
    //     };


}

fn main() {
    let listener = createListener("127.0.0.1", 9000);

    for streamResult in listener.incoming() {
        match streamResult {
            Ok(mut stream) => {
                let rcBox = Arc::new(Mutex::new(stream));
                goWriteRoutine(rcBox.clone());
                goReadRoutine(rcBox.clone());
            }
            Err(e) => {
                println!("e:{:#?}", e);
            }
        }
    }
}
