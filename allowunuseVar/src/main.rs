 
 #![allow(unused_variables)]

 fn main() {
let s = "foo
    bar";

    assert_eq!("foo\n    bar", s);

    let mut msg = "wen";
    let mut sliceIns = &mut msg[1..];

    sliceIns[0]='a';

    println!("res:{}",sliceIns);

}
