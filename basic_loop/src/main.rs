

fn  printLoop(i :i32){
    println!("printLoop:{}",i);
}

fn main() {
    let mut intArr = vec![1,2];
    //ensure size
    for i in 1..10 {
        intArr.push(i);
    }
    println!("vec={:#?}",intArr);


    let mut count =0;
    loop {
         if count>10
         {
            break;
         }
         count =count+1;
    }
   println!("result={}",count);

  
   let mut  loopFalg =0u8;
   while loopFalg<20 {
        printLoop(loopFalg as i32);
        loopFalg = loopFalg+1;
   }


}
