
fn change(a:&mut i32){

    *a =100;

}

fn change2(b:&mut i32){
    *b = 5000;
}


fn main(){
    // let mut a:i32 = 5;
    // change(&mut a);
    // change2(&mut a);

    // {
    //   let mut b = a;
    //   b = 5000;
    // }

    // {
    //     let mut b = a;
    //     b = 8000;
    //   }
    // println!("{}",a);
   
    // //&a先转成raw指针，然后再把指针转成usize，这个可以print的
    // let addr = &a as *const i32 as usize;
    // println!("addr：0x{:X}",addr);

   
    // print!("addr:0x{:X}",addr);

    // //为了验证刚才的地址是不是正确的，我们修改这个指针指向的数据
    // //pa就是addr对应的raw指针
    // let pa = addr as *mut i32;
    // //解引用，*pa其实就是&mut a了，给他赋值100
    // unsafe{*pa = 100};

    

    // //打印a，可以看到a已经变成100了
    // println!("value:{}",a);


    let m =50;
    let addr_usize = &m as *const i32 as usize;

//               usize
// rust c  style     rust c style --->unsafe{}      
//rust &        


    let  addr_rust_c_ptr = addr_usize as *mut i32; // rust c ptr ---> usize ---->rust mut c  ptr  in unsafe block{},usize 是转换的中转站
    //以下演示，rust C风格的指针，只能在 unsafe块里面使用


    let aa =unsafe{
        *addr_rust = 5000
    }
    // println!("prt:{:X}, value={}",addr_usize,*addr_rust);addr_rust是

}
