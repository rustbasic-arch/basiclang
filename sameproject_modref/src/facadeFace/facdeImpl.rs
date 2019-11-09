
use crate::msg::msgImpl::sendMsg;
use crate::auth::authImpl::login;
pub fn doMediator(){
    login();
    sendMsg();
}