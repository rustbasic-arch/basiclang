
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Write;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;



fn spawnWriteRouine(safeBox:Arc<Mutex<TcpStream>>){
    let msg = "server proto";
    thread::spawn(||{


    });


}

fn createListener(ip:&str,port:i32)->TcpListener
{
    let addr = format!("{}:{}",ip,port);
    let listener = TcpListener::bind(addr).unwrap();
    listener
}

fn main() {



    
    let value = loop{
        let mut inputStr = String::new();
        std::io::stdin().read_line(&mut inputStr).expect("not a number");
        let trimmed = inputStr.trim();
        match trimmed.parse::<usize>() {
            Ok(i) =>{
                println!("input:{}",i);
                if i==100{
                    break 10000;
                }
    
            } ,
            Err(..) => println!("this was not an integer: {}", trimmed),
        }
    

    };

    println!("value:{}",value);


}
