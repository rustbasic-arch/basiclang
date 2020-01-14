

fn getMax<'a>(a:&'a i32,b:&'a  i32)->& 'a i32{
     
     match a>b {
        true =>{
               a
         },
        _ =>{
               b
         }
     }

}



fn main() {
     let msg = "wen wei ping";

     let xing  = &msg[0..3];

     println!("xing:{:?}",xing);

    let love_china = "忠犬ハチ公gdfdf";

     for (i,byte) in love_china.as_bytes().iter().enumerate(){
          println!("i:{},byte:{}",i,byte);
     }


     for (i,ch) in love_china.chars().enumerate(){
          println!("i:{},charName:{:>5}",i,ch);
     }

   

      let b = getMax(&100,&200);

      println!("getMax:,max(100,200) ,:{}",b);

}
