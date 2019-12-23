use std::thread;


// fn openRef(r:&mut i32){

//   println!("openRef r:{}",r);
// }

// fn openRef2(r:&mut i32){
//     println!("openRef2 r:{}",r);
//   }

// fn openRef3(r:&mut i32){
   
//     let mut aa = *r;//a is new zone will copy value from *r
//     println!("ptr &aa={:p},r:{:p}",&aa,r);
    
//     aa =1000;
//     // let b  =  Box::new(*r);
//     let h =thread::spawn(move ||{
//       aa= 5000;
//         println!("openRef3 r:{}",aa);
//     });
//     h.join();
//     thread::sleep_ms(1000);

// }


fn aa (i:&mut i32){
  *i=5000;
  println!("a:={}",i);
}

fn callB(mut b:i32)
{
  b =8888;
  println!("in callB:{}",b);
}
fn main() {
    let mut a = 100;
    let mut b =a;
    callB(b);
    println!("after callB:{}",b);
    println!("a={:p},b={:p}",&a,&b);

    aa(&mut a);
    println!("c:={}",a);
}
