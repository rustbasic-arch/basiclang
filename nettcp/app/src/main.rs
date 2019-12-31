use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::thread;

fn  ConnWriteRoutine(mut stream:TcpStream){

   let msg = "server proto  msg";
   loop{
      let result = stream.write(msg.as_bytes()).unwrap();
      match result {
          Ok(expr) => {
              println!("")
          }},
          Err(e) => {


          },
      }
      std::thread::sleep_ms(1000);
      println!("has write:num= {}",num);
   }
    

fn onRecvConn(listener:TcpListener){
   for stream in listener.incoming()
   {
      match stream {
          Ok(mut expr) => {
          let h = std::thread::spawn(move ||{
            ConnWriteRoutine(expr);
         });
         h.join();
      }
          ,
          Err(e) => {
          println!("error :{}",e);

          }
      }

   }

}

fn main() {

    let ip= "127.0.0.1:9000";
    let listener = TcpListener::bind(ip).unwrap();
    onRecvConn(listener);

}
