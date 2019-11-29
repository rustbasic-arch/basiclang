

pub mod friend;
pub mod auth;
use friend::friendimpl::addFriend;

fn main() {
    println!("Hello, world!");
    addFriend();
}
