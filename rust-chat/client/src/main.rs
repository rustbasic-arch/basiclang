
use std::io;
use std::net::TcpStream;
use std::str;
use std::io::{Write,Read};
use std::thread;
use std::result::Result;

// fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()> {

//     let mut buf = vec![0u8;1024];
//     loop {
//         // 对比Listener，TcpStream就简单很多了
//         // 本次模拟的是tcp短链接的过程，可以看作是一个典型的HTTP交互的基础IO模拟
//         // 当然，这个通讯里面并没有HTTP协议 XD！
//         let mut stream = TcpStream::connect(&addr).unwrap();
//         let msg = "WaySLOG comming!".as_bytes();
//         // 避免发送数据太快而刷屏
//         thread::sleep_ms(100);
//         let rcount = try!(stream.write(&msg));
//         let _ = try!(stream.read(&mut buf));
//         println!("{:?}", &buf[0..rcount]);
//         buf.clear();
//     }
//     Ok(())
// }



fn main() {
    let msg = "WaySLOG comming!".as_bytes();
    let mut buf = vec![0u8;1024];
    let mut stream = TcpStream::connect("127.0.0.1:20080").unwrap();
    let mut input = String::new();
    let mut responce = String::new();

   let rcount = stream.write(&msg);
//    println!("{:?}", &buf[0..rcount]);
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                stream.write(input.as_bytes());
               // let resp = str::from_utf8(stream.read(responce));
            }
            Err(error) => println!("error: {}", error),
        }
    }
    
} // the stream is closed here