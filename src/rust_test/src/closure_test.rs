#[allow(unused)]
#[derive(Debug)]
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 惰性求值
    // 只有第一次调用有效
    // 后续继续调用都是返回的第一次的值
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


pub fn main() {
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 }; // 未知参数类型，会在第一次使用时推断这些类型
    let add_one_v4 = |x| x + 1;

    println!("add_one_v1 {}", add_one_v1(1));
    println!("add_one_v2 {}", add_one_v2(1));
    println!("add_one_v3 {}", add_one_v3(1));
    println!("add_one_v4 {}", add_one_v4(1)); // add_one_v4后面只能接收i32类型参数
    // println!("add_one_v4 {}", add_one_v4("123")); // error
    println!("add_one_v4 {}", add_one_v4(125));

    // error
    // let c = Cacher <|a| {
    //     a
    // } > {};
    // println!("Cacher value {:?}", c.value);
    // println!("Cacher calculation {}", c.calculation(20)); // error

    let mut c = Cacher::new(|i| i * 10);
    println!("c.value {:?}", c.value); // None
    println!("c.value {:?}", c.value(20)); // 200
    println!("c.value {:?}", c.value); // Some(200)
    println!("c.value {:?}", c.value(30)); // 200


    let a = 10;
    let closure_test = |b: i32| b == a; // 闭包访问外部变量
    println!("10 == 20 : {}", closure_test(20)); // false
    println!("10 == 10 : {}", closure_test(10)); // true

    // 函数不能访问外部变量
    /* fn read_a(b: i32) -> bool {
        a == b
    } */

    // println!("10 == 20 : {}", read_a(20)); // error


    let x = vec![1, 2, 3];

    println!("can use x here: {:?}", x);
    let equal_to_x = move |z| z == x; // 闭包移动了x的所有权，外面的x不能再使用了
    // println!("can't use x here: {:?}", x); // error 不能再使用了

    let y = vec![1, 2, 3];
    println!("can use y here: {:?}", y);
    assert!(equal_to_x(y)); // true 说明闭包里面还有x
    // println!("can't use y here: {:?}", y); // error 不能再使用了
}