fn apply<F:Fn()>(func:F){

    func();

}

// fn apply2<Function,A,B>(fun:Function,p1:A,p2:B)where
//                                  Function:Fn(i32,i32),A:i32,B:i32{
//         fun(p1,p2);

// }


fn main() {
    // println!("Hello, world!");

    // let f = ||{
    //         println!("clouse");
    // };

    // apply(f);

    // let add = ||{
        
    // }

    let value = (0..10).map(|x| {
        fn f(y: u32) -> u32 {
        y*y
        }
        let z = f(x+1) * f(x+2);
        z*z
        });

        println!("value={:?}",value)

        // for i in value.iter(){
        //     println!("each:{}",i);
        // }
        
}
