

#[derive(Debug)]
struct ContainerListener {
     func:fn (a:i32,b:i32)->i32;
}

impl ContainerListener {
     
     fn exec(&self){
         (self.func)(100,200);
     }

     fn defaultAdd(a:i32,b:i32)->i32
     {
         println!("defaultAdd....");
         a+b
     }

}

fn add(a:i32,b:i32)->32
{
    println!("defaultAdd....");
    a+b
}


fn main() {
    let execCon = ContainerListener{
            func:add
    };
    execCon.exec();
    
    println!("Hello, world!");
}
