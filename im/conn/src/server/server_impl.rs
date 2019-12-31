
use std::net::{TcpListener,TcpStream};


fn createListener(ip:&str,port:i32)->TcpListener{
   let addr = format!("{}:{}",ip,port);
   let listener = TcpListener::bind(addr).unwrap();
   listener
}
