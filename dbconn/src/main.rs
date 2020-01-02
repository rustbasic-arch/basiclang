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
    // let basemsg = String::from("weibo");
    let basemsg2 = "weibo";
    // let msg = format!("idx:{},msg:{}",idx,&basemsg);
    let msg = format!("idx:{},msg:{}",idx,basemsg2);//format!("{}",&str); &str type needed
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


fn incommingConn(lis:Arc<Mutex<TcpListener>>,idx:i32){
    println!("开启侦听端口:{}",idx);
     for streamBox in lis.lock().unwrap().incoming(){
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
    let  mut listenerMap = HashMap::new();
    for i in 9000..9100
    {
        let listener = createListener("127.0.0.1",i);
        let k =format!("{}:{}","127.0.0.1",i);
      
        let safeBox = Arc::new(Mutex::new(listener));
        let pre_InthreadBox1 = safeBox.clone();//pre clone before in thread
        let pre_InthreadBox2 = safeBox.clone();//can dispath ability
        let pre_InthreadBox3 = safeBox.clone();//can dispath ability
        let h = thread::spawn(move || {
            incommingConn(pre_InthreadBox1,i);
        });
        listenerMap.insert(k, safeBox);// pass h  but h not implemenets copy trait ,u know Cell has implements copy tra
        handlers.push(h);//  pass h
    }

    for 

    for  h in handlers{
            h.join().unwrap();
    }
   

}
