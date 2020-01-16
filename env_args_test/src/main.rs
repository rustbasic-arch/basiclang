



fn main() {
   
     for arg in std::env::args(){
         println!("cmd list :\n{:>100}",arg);
     }
     let commands:Vec<String> = std::env::args().collect();
     if (commands.len()>1)
     {
        println!("{}\n",commands[1]);
     }else{
         println!("haha ");
     }
    
}
