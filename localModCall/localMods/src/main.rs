pub mod auth;
pub mod friend;

fn main() {

    use auth::authImpl::login;
    use friend::friendImpl::addFriend;

    login();
    addFriend();
}
