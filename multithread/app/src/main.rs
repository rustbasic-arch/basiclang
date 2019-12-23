//! mio+threadpool

#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate mio;
extern crate threadpool;
extern crate num_cpus;

use std::thread;
use std::str::FromStr;
use std::time::Duration;
use std::io::{Read,Write};
use threadpool::{ThreadPool,Builder};
use mio::*;
use mio::tcp::{TcpListener, TcpStream};

fn main() {
    simple_logger::init().unwrap();
    let server_handle=run_server(None);
    server_handle.join();
}

fn run_server(timeout: Option<Duration>)->thread::JoinHandle<()>{
    let handle=thread::spawn(move||{
        let num_cpus=num_cpus::get_physical();
        let pool=Builder::new().num_threads(num_cpus).thread_name(String::from("threadpool")).build();

        const SERVER: Token = Token(0);
        let addr = "127.0.0.1:12345".parse().unwrap();
        let server = TcpListener::bind(&addr).unwrap();

        let poll = Poll::new().unwrap();
        poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();
        let mut events = Events::with_capacity(1024);
        loop {
            match poll.poll(&mut events, timeout){
                Ok(size)=>{
                    trace!("event size={}",size);
                    if size<=0{
                        break;
                    }
                },
                Err(e)=>{
                    error!("{}",e);
                    break;
                }
            }
            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (stream,_) = server.accept().unwrap();
                        pool.execute(move ||{
                            simple_echo(stream);
                        });
                    },
                    _ => unreachable!(),
                }
            }
        }

        pool.join();
    });

    handle
}

fn simple_echo(mut stream:TcpStream) {
    info!("New accept {:?}", stream.peer_addr());
    let mut buf = String::new();
    if let Err(e) = stream.read_to_string(&mut buf) {
        error!("{}", e);
    }

    thread::sleep_ms(1000); //加上延时是为了验证线程池工作
    info!("server receive data: {}", buf);
    stream.write_all(buf.as_bytes());
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_server(){
        simple_logger::init().unwrap();
        let server_handle=run_server(Some(Duration::new(10,0)));
        thread::sleep_ms(1000);
        let client_handle=run_client(4);
        client_handle.join();
        server_handle.join();
    }

    fn run_client(num: usize)->thread::JoinHandle<()>{
        let handle=thread::spawn(move||{
            let mut ths=Vec::new();
            for id in 0..num{
                let h=thread::spawn(move||{
                    client(id);
                });
                ths.push(h);
            }

            for h in ths{
                h.join().unwrap();
            }
        });

        handle
    }

    fn client(id: usize){
        let mut stream = std::net::TcpStream::connect("127.0.0.1:12345").unwrap();
        let mut data=format!("client data {}",id);
        stream.write_all(data.as_bytes());
        let mut buffer=String::new();
        stream.read_to_string(&mut buffer);
        info!("client {} received data:{}",id,buffer);

        info!("connect {} end!",id);
    }
}