use std::fmt::Debug;
pub fn matchOpt<T:Debug>(o:Option<T>){
    
    match o {
        Some(fetchValue) => println!("{:?}",fetchValue),
        None => println!("nothing"),
    }
    

}

