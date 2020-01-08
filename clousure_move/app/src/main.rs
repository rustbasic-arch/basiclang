

use std::ops::FnOnce;
use std::time::SystemTime;
use std::thread;

fn testMove() {
    println!("Hello, world!");

    let mut x = 6;
    
    //不写move编译器，认为 下面println 还有个不可变引用呢,而采用move，则告诉编译器,这两个地方不会同时发生，因此采用了移动所有权
        let mut  f=  move   |i:i32|{ //move or  not move :result is ,here is copy
            // x;//x被捕获过程 capture incoming
            // x被捕获过程 x=x+10;modify value
                x+=10;
        };
        f(1000);
    
    println!("x={}",x);
}



//ownship test
fn main()
{
    // let mut num = 1000;
  
    // //asyn will use and not 
    // let changeFn = |input:i32|-> i32{
    //     num+input  //caputure not change ,compiler infer here is Fn ,不会发生所有权转移
    // };

    // let y = &&&&& num; // if here let y =&mut num
    // println!("y:{}",y); //fore deref polymorphic by &
    testMove();
 
}


