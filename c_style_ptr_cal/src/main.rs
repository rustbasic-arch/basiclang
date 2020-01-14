
struct Stu {
    age :i32,
    score:i8,
}

impl  Stu {
     fn new()->Self{
        Stu{age:20,score:100}
     }
}

fn main() {

    let s  = Stu::new();
    let base_Ptr = &s as *const Stu as   usize;

    let age_ptr =  &(s.age)  as  *const i32 as usize;
    let score_ptr =  &(s.score)  as *const i8 as  usize;


    let mut msg = String::new();
    msg.push_str("english\n");
    msg.push_str("french\n");
    msg.push_str("math");

    println!("age offset{} ,score offset:{}\n",age_ptr-base_Ptr,score_ptr-base_Ptr);
    println!("{}",msg);
    println!("{}",msg);











    //test for
    for i  in (0..10).rev(){
        println!("i:{}",i);
    }

    let arr = [5;10];

    for  e in arr.iter(){
        println!("e:{}",e);
    }





}


