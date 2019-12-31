

trait Run {
    fn run(&self);
    fn exec(&self);
}

trait Eat {
    fn eat(&self);
    fn exec(&self);
}


struct Person(i32);

//extends Run
impl Run for Person {
    fn run(&self)
    {
        println!("Person runing");
    }
    fn exec(&self)
    {
        println!("Person run exec");

    } 
}
//extends Eat
impl Eat for Person {
    fn eat(&self)
    {
        println!("Person eating");
    }
    fn exec(&self)
    {
        println!("Person Eat exec");
    }
}

trait Oss {
    fn exec(&self);

}

//extends Oss
impl Oss for Person{
    fn exec(&self)
    {
        println!("oss exec downloading");
    }

}



fn main() {
    let p = Person(0);
    //  p.exec();//error exec has two version ,can use interface trait to instead of
     p.eat();//ok

     let eatPtr = &p as &Eat;
     eatPtr.exec();

    //  let runPtr = &p  as &Run;
    //  runPtr.run();
    //  runPtr.exec();


    //  let ossPtr = &p as &Oss ;
    //  ossPtr.exec();

}
