
pub mod auth;

pub mod friend;

fn main() {

    auth::authImpl::login();
    let friendName = "hello".to_string();
    friend::friendImpl::addFriend(&friendName);

    drop(friendName);

}
