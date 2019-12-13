use std::option::Option;
use std::net::{TcpListener,TcpStream};
use std::fs;
use std::io::prelude::*;
use std::ops::Add;
//orient src
struct Point<T>{
   x: T,
   y: T,
   z: T
}

impl<T>  Point<T> {
    //out ref
    fn distance(&self)->&T{
        &self.x
    }
}

struct  Int32 {
    value :i32,
};

impl Add<Int32> for Int32 {
    fn add(mut self:&Self, rhs: Int32) -> &Self
    {
        self.value += rhs.value;
        self
    }
}

fn add<T:Add>(a:T,b:T)->T{
    a+b
}


fn main() {
   
    let i32Box:Option<i32>=Some(10_i32);

    print!("{:#?}",i32Box);

    match i32Box{
        Some(expr) => 
        print!("{}",expr)
        ,
        None =>
        println!("yes")
         ,
    }

    let  p = Point{x:10,y:10,z:10};
    let res =  p.distance();
    println!("res:{}",res);
    // let res2 = add(10,20);
    // println!("{}",res2);
    
   let a1 = Int32{value:10};
   let a2 = Int32{value:200};

   println!("{}",a1+b2)



}
