


fn main() {
    
    let mut dyArr = vec![1,2,30,0,4];

    for i in dyArr.iter(){
        println!("i:{}",i);
    }

    dyArr.push(10);
  
    for (i,v) in dyArr.iter().enumerate(){
        println!("k:{},b:{}",i,v);
    }


    let v1 = vec![1, 2, 3];
let mut v1_iter = v1.iter();
if let Some(v) = v1_iter.next() {
	println!("{}", v); //1
}

if let Some(v) = v1_iter.next() {
	println!("{}", v); //2
}

if let Some(v) = v1_iter.next() {
	println!("{}", v); //3
}

if let Some(v) = v1_iter.next() {
	println!("{}", v); //3
} else {
	println!("Reached end!") //printf "Reached end"
}


 println!("i32size:{}",std::mem::size_of::<i32>());

}
