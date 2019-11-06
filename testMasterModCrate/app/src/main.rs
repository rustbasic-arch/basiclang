
pub mod auth;
pub mod friend;

extern crate mylib;
use mylib::authmod::authImpl::createAuth;

fn main() {

    auth::authImpl::login();
    let friendName = "hello".to_string();
    friend::friendImpl::addFriend(&friendName);

    drop(friendName);
    createAuth();

 

}
