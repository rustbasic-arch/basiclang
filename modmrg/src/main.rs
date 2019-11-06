
pub mod auth;
pub mod nosubmod;

fn main() {
    println!("Hello, world!");
    auth::addAuth();
 
    let res = nosubmod::add(10,20);

}
