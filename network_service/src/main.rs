use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

fn spawnWriteRouine(safeBox: Arc<Mutex<TcpStream>>) {
    let msg = "server proto";
    thread::spawn(|| {
        let writeExecutor = safeBox.lock().unwrap();
        loop {
            thread::sleep_ms(1000);
            match writeExecutor.write(msg.as_bytes()) {
                Ok(size) => {}
                Err(e) => {

                },
            }
        }
    });
}


fn spawnReadRoutine(safeBox:Arc<Mutex<TcpStream>>){


}


fn createListener(ip: &str, port: i32) -> TcpListener {
    let addr = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(addr).unwrap();
    listener
}

fn spawnListener(port: i32) {
    let listener = createListener("127.0.0.1", port);

    for streamReult in listener.incoming(){
        match streamReult {
            Ok(mut stream)=>{
                let safeBox = Arc::new(Mutex::new(stream));
                spawnWriteRouine(safeBox.clone());
                spawnReadRoutine(safeBox.clone());
            },
            Err(e)=>
            {

            }

        }

    }

}

fn main() {
    let handlers = vec![];
    for port in 9000..9100 {
        spawnListener(port);
        let  h=  thread::spawn(move || {
            spawnListener(port);
        });
        handlers.push(h);
    }

    let value = loop {
        let mut inputStr = String::new();
        std::io::stdin()
            .read_line(&mut inputStr)
            .expect("not a number");
        let trimmed = inputStr.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => {
                println!("input:{}", i);
                if i == 100 {
                    break 10000;
                }
            }
            Err(..) => println!("this was not an integer: {}", trimmed),
        }
    };

    println!("value:{}", value)