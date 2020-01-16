



fn main() {

    let mut msg = String::from("art\n");
    msg.push_str("math\n");
    msg.extend(&['s','p','o','r','t','s','\n']);

    println!("msg:{}",msg);

    let mut msgclone = msg.clone();

    
    msgclone.extend(&['a','b','c','\n']);

    println!("msgclone:{}",msgclone);

    for  (i,by) in msgclone.as_str().as_bytes().iter().enumerate(){
        println!("i:{},b:{}",i,by);
    }

    for  (j,ch) in msgclone.as_str().chars().enumerate(){
        println!("j :{},ch:{}",j,ch);
    }
   
   let ip = "192.168.1.108";
   let port = 1000;
   
   let addr  = String::from(ip)+":"+&port.to_string()+"say:\n"+&msgclone;
   
   println!("addr:{}",addr);

   


//    let vecSlice = vec![1,2,3,4,5];

   let numbers = vec![1,2,3,4,5]; //slice implements iterator trait
    for n in numbers {
        println!("{} is a number!", n);
    }



}
