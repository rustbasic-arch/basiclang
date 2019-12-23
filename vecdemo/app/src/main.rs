

fn addOneVec(v:&mut Vec<i32>){

    v.push(1);

}



fn addOneVec2(v:&mut Vec<i32>){

    v.push(2);

}

fn main() {
     let mut  v =vec![10,12];
     addOneVec(&mut v);
     addOneVec2(&mut v);

     let arrs = [1,2,31];
     
     let len = arrs.len();
     for i in 0..len {
        println!("value:{}",arrs[i]);
     }

     for  j in arrs.iter(){
         println!("value:{}",j);
     }

     for (index, value) in arrs.iter().enumerate(){
        println!("index {},value:{}",index,value);
     }

     let mut macroArr = vec![1;10];

     for k in macroArr.iter(){
         println!("k={}",k);
     }

     macroArr.extend([100,200,300].iter().cloned());
 

     

}
