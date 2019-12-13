trait MyTrait {
    fn area(&self)->i32;
}



impl MyTrait for i32 {
    fn area(&self)->i32
    {
       let s =   *self as i32;
       s*s
    }
}

trait Command {
    fn exec(&self){
        self.say();
    }
    fn say(&self);
}

impl Command for f64 {
    fn say(&self){
        println!("say method execute ...value :{}",*self);
    }
}


fn recv((x,y):(i32,f64))
{
    println!("x{},y{}",x,y);
}

fn printStr(s:&str)
{
    println!("print str:{}",s);
}

fn main() {
  let res =  5.area();
  println!("res={}",res);
    
   1024.23f64.exec();
   let a = (23,23.36);
   let  (x,y):(i32,f64) =  a;
   
   println!("{},{}",x,y);
   recv( a);

   printStr("hello world ");
   
}
