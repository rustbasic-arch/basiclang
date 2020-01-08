use std::io;


fn main() {



    
    let value = loop{
        let mut inputStr = String::new();
        std::io::stdin().read_line(&mut inputStr).expect("not a number");
        let trimmed = inputStr.trim();
        match trimmed.parse::<usize>() {
            Ok(i) =>{
                println!("input:{}",i);
                if i==100{
                    break 10000;
                }
    
            } ,
            Err(..) => println!("this was not an integer: {}", trimmed),
        }
    

    };

    println!("value:{}",value);


}
