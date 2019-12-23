




fn main() {
    let bytes = [11u8,22u8,33u8,44u8];

    unsafe{
        let b = &bytes as (*const [u8]);
    }
   
}
