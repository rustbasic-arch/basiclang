
trait Eat<T>{
    fn eat(&self)->T;
}

struct Animal();

impl Eat<i32> for Animal { //eat 签名  的形成和 模板参数i32+struct 名字空间确定+eat
    fn eat(&self)->i32{
        println!(" animal eat....\n");
        30
    }
}

//上面实例化  模板参数
impl Eat<f64> for Animal {////eat 签名  的形成和 模板参数f64+struct 名字空间确定+eat
    fn eat(&self)->f64{
        println!("Animal   eat->f64  overload ");
        10.26_f64
    }
}



trait ConverterA{
 
    type Output;
    type Input;

    fn convert(&self,idddn:Self::Input)->Self::Output;
}

trait ConverterB{
 
    type Output;
    type Input;

    fn convert(&self,idddn:Self::Input)->Self::Output;
}

impl ConverterB for Animal{ 
    type Output =f64;
    type Input = f64;
    fn convert(&self,idddn:Self::Input)->Self::Output
    {
        1000.23 as f64
    }

}




impl ConverterA for Animal{
  
    type Output =i32;
    type Input = i32;
    fn convert(&self,idddn:Self::Input)->Self::Output
    {
        1000 as i32
    }

}

//综上 一致性标识 问题：
//模板参数情况：函数签名形成
//trait name +  changePart(template param) +method
//关联参数情况：函数签名形成;(关联参数不会影响函数签名，所以容易有冲突 ，函数签名一致性问题)

//trait name_(ChangePart) +   method 

// 但是
// 模板参数+struct 名字空间确定+method
//通过关联参数，会出现 函数重复定义问题

fn main() {
    let ani = Animal();
    let a32:i32 = ani.eat();

    let a64:f64 = ani.eat();


    let e :&Eat<i32> = &ani as &Eat<i32>;
    let a32:i32= e.eat();

    let ef :&Eat<f64> = &ani as &Eat<f64>;
    
    let a64:f64= ef.eat();


    println!("a32:{}:a64:{}",a32,a64);

//下面报错；两个同时出现 convert(10000)   和convert(21.63_f64) 同时出现，由此说明

//convert  的形成和 模板参数+struct 名字空间确定+method
//     let res =   ani.convert(10000);

//     println!("i32:{}",res);

//     let resf = ani.convert(21.63_f64);

//  println!("resf:{}",resf);

}
                                                                                                             