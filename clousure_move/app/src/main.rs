

use std::ops::FnOnce;
use std::time::SystemTime;
use std::thread;
fn testMove() {
    println!("Hello, world!");

    let mut x = 6;
    {
        let mut  f= move |i:i32|{ //move or  not move 
                x+=10;
        };
        f(1000);
    }
    println!("x={}",x);
}

//ownship test
fn main()
{
    let mut num = 1000;
  
    //asyn will use and not 
    let changeFn = |input:i32|-> i32{
        num+input  //caputure not change
    };

    let y = &&&&& num; // if here let y =&mut num
    println!("y:{}",y); //fore deref polymorphic by &
    
}

