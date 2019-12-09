

macro_rules! CmdCreator {
    ($funcName:ident) => (
        fn $funcName(){
            println!("hello");
        }

    )
}