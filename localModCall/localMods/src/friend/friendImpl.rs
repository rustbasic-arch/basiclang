


pub fn addFriend(){
    //说明!!! use crate ::auth::authImpl::login;try error (maybe only cross lib-style is right(  now here is error))
    // use super::super::auth::authImpl::login;//说明!!! (local crate is ok,run ok)
    use ::auth::authImpl::login;
    println!("addFriend...start");
    login();
    println!("addFriend...end ");
}