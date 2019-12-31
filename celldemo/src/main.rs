use std::cell::Cell;//copy
use std::cell::RefCell;//move



fn main() {
    let a = RefCell::new(5);
    
    *a.borrow_mut()=100;

    println!("{}",a.borrow());
 
}
