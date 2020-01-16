

struct MessageDes<'a>{
     uuid:&'a str,
     content:String,
}

impl<'a> MessageDes<'a>{
    fn sendMsg<'b>(&self,session:&'b str){
        println!("uuid:{},send msg:{}",self.uuid,&self.content);
    }

    fn new()->MessageDes<'a>{
        let msg = String::from("fdsfsdfsdfsdfsdfsdf");
        MessageDes::<'a>{uuid:"1234abcd",content:msg.clone()}
    }
}
//全局变量声明和定义
const sessionId :&'static str = "8011";

fn main() {
 
  
    let m =  MessageDes::new();

    m.sendMsg(sessionId);


}
