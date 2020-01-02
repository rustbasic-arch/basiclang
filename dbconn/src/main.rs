/***
 * multi thread 
 * multi port
 * */


use std::net::{TcpListener,TcpStream};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::io::ErrorKind;
use std::thread;
use std::io::Write;



fn  createListener(ip:&str,port:i32)->TcpListener{
    let addr = format!("{}:{}",ip,port);
    let lis = TcpListener::bind(addr).unwrap();
    return lis;
}


fn doWriteRoutine(safeBox:Arc<Mutex<TcpStream>>,idx:i32)
{
    let basemsg = "微博";
    let msg = format!("idx:{},msg:{}",idx,basemsg);
    thread::spawn(move || {
        let mut writeExecuter = safeBox.lock().unwrap();
        loop{
                thread::sleep(Duration::from_secs(1));
                match writeExecuter.write(msg.as_bytes())
                {
                    Ok(size)=>{

                    },
                    Err(e)=>{
                        match e.kind(){
                            ErrorKind::ConnectionAborted=>
                            {   
                                println!("client exit :reason:{:#?}",e);
                                break ;
                            },
                            _=>{
                                println!("client error :reason:{:#?}",e);
                            }


                        }
                    }
                }
        }
    });



}


fn incommingConn(lis:TcpListener,idx:i32){
    println!("开启侦听端口:{}",idx);
     for streamBox in lis.incoming(){
        match streamBox 
        {
            Ok(stream) =>{
                    println!("有一个客户进入....ip:{:#?}",stream.peer_addr());
                    let safeBox = Arc::new(Mutex::new(stream));
                    doWriteRoutine(safeBox,idx);
            },
            Err(e)=>{


            }


        }


     }
}

//multi
fn main() {
    let  mut handlers = vec![];
    // let  listenerMap = HashMap::new();
    for i in 9000..9100
    {
        let listener = createListener("127.0.0.1",i);
        let h = thread::spawn(move || {
            incommingConn(listener,i);
        });
        
        handlers.push(h);
    }


    for  h in handlers{
            h.join().unwrap();
    }
   

}
