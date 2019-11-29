pub mod libr;
use libr::librim::matchOpt;

fn main() {
    println!("Hello, world!");
    let a:Option<i32> =Some(100);
    let b:Option<u32> = None;
    matchOpt(a);
    matchOpt(b);
    
}
