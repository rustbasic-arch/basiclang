use std::str::FromStr;
use std::num::ParseIntError;
#[warn(non_snake_case)]

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}


impl Point{
 
    fn new()->Result<Self,ParseIntError>{
        Ok(Point{x:10,y:10})
    }
}


impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(",")
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr})
    }
}



fn main(){

    let p = Point::from_str("(1,2)");
    assert_eq!(p.unwrap(), Point{ x: 1, y: 2} );


    let p111 = Point::new().unwrap();
    // if let Point{x:i32,y:i32} = p111 {
    //     println!("aaaa:{},bbb:{}",x,y);
    // }

    // println!("x:{},y:{}",p111.x,p111.y);
 

}