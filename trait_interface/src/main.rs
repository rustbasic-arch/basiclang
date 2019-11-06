struct Test1{
	data : i32,
	size : i32,
}
struct Test2{
	age : i32,
	name: String,
}
//定义一个trait（接口/特性）
trait Show{
        //self关键字类似于c++的this指针
        //Self代表的是实现该方法的具体【类型】
	fn show(&self);
	fn new()->Self;
} 
impl Show for Test1{
	fn show(&self){
		println!("{},{}", self.data, self.size);
	}
	fn new()->Self{
		Test1{
			data:2019,
			size:25,
		}
	}
}
impl Show for Test2{
	fn show(&self){
		println!("{},{}", self.age, self.name);
	}
	fn new()->Self{
		Test2{
			age : 30,
			name:String::from("vigiking"),
		}
	}
}
//定义一个测试函数
//t:&impl Show 意思是实现了该接口的类型对象
fn test_trait(t:impl Show){
	t.show();
}
fn main() {
	let t1 = Test1::new();
	let t2 = Test2::new();
	test_trait(&t1);
	test_trait(&t2);
	println!("------------");
	t1.show();
	t2.show();
}