
#[derive(Debug)]
struct Point(f32,f32,f32);

impl Point  {
     pub fn new()->Self
     {
        Point(100.23,200.2,600.6)
     }
}

fn main() {
    let p = Point::new();
    println!("res:{:#?}",p);

    

}
