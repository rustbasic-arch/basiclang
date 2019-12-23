use std::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use std::io::ErrorKind;
use std::io::Write;
use std::io::Read;
use std::result::Result;

fn goRead(conn:&TcpStream){
    
    let h = std::thread::spawn(move ||{
        //let mut bt = [0u8,1024];
        loop{
         //    conn.read(&mut bt);
        }

    });

    h.join();

}

fn goWrite(conn:&mut TcpStream){
   let message = "json object".as_bytes();
   
   let h = std::thread::spawn(move || {
    conn.write(b"message");
    std::thread::sleep_ms(2000);
   });
   h.join();
   

}


fn createListener(ip:&str,port:i32)->TcpListener
{
    let addr = format!("{}{}",ip,port);
    let listener = TcpListener::bind(addr).unwrap();
    listener
}

fn listenerRoutine(lis:&TcpListener){
      for streamConn in lis.incoming(){
          match streamConn {
              Ok(mut streamConn) => {
                goRead(&streamConn);
                goWrite(&mut streamConn);
              },
              Err(e) => {
                    println!("e:{:#?}",e);
              },
          }

      }
}

fn main() {

    let  listener = createListener("127.0.0.1",9000);

    listenerRoutine(&listener);
    
 
}
