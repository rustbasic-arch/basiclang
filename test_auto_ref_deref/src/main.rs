struct Number(i32);

fn onProcess(n: &Number) {
    println!("res:{}", n.0);
}

fn onChange1(n: &mut Number) {
    n.0=110;
    println!("res:{}", n.0);
}
fn onChange2(n: &mut Number) {
    n.0=120;
    println!("res:{}", n.0);
}

fn main() {
    let mut n = Number(100);
    {
        let nRef = &mut n;
        nRef.0 = 5000;
    }

    {
        let nRef2 = &mut n;
        nRef2.0 = 10000;
    }
    onChange1(&mut n);
    onChange2(&mut n);
    onProcess(&mut n);
}
