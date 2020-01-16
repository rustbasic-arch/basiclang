
struct  Os{

}

impl Os {

    fn getOsName()->&'static str
    {
        if cfg!(target_os="windows")
        {
            return "windows"
        }else if cfg!(target_os="linux")
        {
            return "linux"
        }


        return "other"
    }

}


fn main(){

    println!("os:{}",Os::getOsName());

}