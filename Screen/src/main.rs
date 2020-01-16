
trait Drawable{
    fn draw(&self);   
}


struct SelectBox{
    x:f64,
    y:f64,
    width:f64,
    height:f64,
    options:Vec<String>
}

impl Drawable for SelectBox{

    fn draw(&self){
     println!(" draw a select box");
    }
}

struct Button{
    x:f64,
    y:f64,
    width:f64,
    height:f64,
    label:String,
}

impl Drawable for Button{
    fn draw(&self){
        println!(" draw a Button ");
       }
}

struct Screen<'a,T:'a+Drawable>{
    components:Vec< Box<&'a T> >,
}


impl <'a,T> Screen<'a,T> where  T:Drawable    //impl <T:Draw>  Screen<T:Draw> {}
{
    fn run(&self)
    {
        for component in self.components.iter()
        {
            component.draw();
        }
    }

}


fn main() {
   
     let screen = Screen{
        components:vec![
            Box::new(&Button{
                x:10.2,
                y:20.2,
                width:102.3,
                height:200.23,
                label:String::from("button1"),
            } as &Drawable),
 
            
            // Box::new(&SelectBox{
            //     x:10.2,
            //     y:20.2,
            //     width:102.3,
            //     height:200.23,
            //     options:vec![
            //         String::from("yes"),
            //         String::from("no"),
            //         String::from("maybe")
            //     ]

            // }),
        ]

     }; 
     screen.run();



}
