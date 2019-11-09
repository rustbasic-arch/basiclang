fn add(p:(i32,i32))->i32{
    return p.0+p.1;
}

fn add2((x,y):(i32,i32))->i32{
    x+y
}


fn main() {
    let mut func :fn((i32,i32))->i32 = add;

    let res = func((10,20));
    println!("res:={}",res);

    func = add2;
    let res2 = func((100,200));
    println!("res2={}",res2);
}
