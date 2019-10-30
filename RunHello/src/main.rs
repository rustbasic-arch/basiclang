

fn main() {
    #[derive(Debug)]
     struct Msg{
         id:i64,
         name:String,
         password:String
     }

     impl Msg {
         fn new(id:i64,name:String,password:String)->Self{
            Msg{
                name,
                id,
                password
            }
         }
         fn setName(&mut self,n:String){
             self.name = n;
         }
         fn print(&self){
             println!("{:#?}",self);
         }
     }

   
     let mut m = Msg::new(123 as i64, String::from("wenwp"),String::from("123456"));
     m.print();
     m.setName(String::from("newton"));
     m.print();
    

}
