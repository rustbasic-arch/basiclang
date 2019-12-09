#[derive(Debug)]
struct Point(f32, f32, f32);

impl Point {
    pub fn new() -> Self {
        Point(100.23, 200.2, 600.6)
    }
}

trait Cmd {
    fn setData(&self);
    fn execute(&self);
}

fn applyCmd(cmd:&Cmd){
    cmd.execute();
}

// fn newCmd(s:&str)-> impl Cmd{


// }

macro_rules! sayHello {
    () => (
        println!("sayHello");
    )
}



fn main() {
    let p = Point::new();
    println!("res:{:#?}", p);

    struct AddCmd {}

    impl Cmd for AddCmd {
        fn setData(&self) {
            
        }
        fn execute(&self) {
            println!("AddCmd execute");
            sayHello!();
        }
    }

    struct SubCmd {};

    impl Cmd for SubCmd {
        fn setData(&self) {}
        fn execute(&self) {
            println!("SubCmd execute");
            sayHello!();
        }
    }


    let add= AddCmd{};

    let sub = SubCmd{};

    applyCmd(&add);
    applyCmd(&sub);

    


    


}
