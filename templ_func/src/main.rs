

fn change(inputValue:&mut i32){

        *inputValue=200;
}   

fn max<T:Copy+PartialOrd>(list:&[T])->T{
    let mut m = list[0];
    let iterator = list.iter();

    for &v in  iterator{
         if v<m{
             m =v;
         }
    }
    m
}

fn largest<T:PartialOrd+Copy>(list:&[T])->T{ //注意，要实现比较和复制的trait才行，否则报错
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}

// fn main(){
//     let number_list = vec![1,2,22,3,42];
//     let arr =[1,8,10,0,23,5];
// 	let r1 = max(&arr);
// 	println!("r1 = {}", r1);
// 	let char_list = vec!['a', 'y', 'c', 'd'];
// 	let r2 = largest(&char_list);
// 	println!("r2 = {}", r2);
// }
 


fn main() {
   let   arrSlice = &[10,1,2,1000,3]; //             
   let  m = max(arrSlice); /// arrSlice= &[10,1,2,1000,3] max(arrSlice);    和arrSlice=[10,1,2,1000,3]  max(&arrSlice);等效，slice 里面存储的都是 &T
 
   println!("max:{}",m);

}
