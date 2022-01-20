#![allow(unused)]
//! # 高级函数和闭包

/// 函数指针（function pointer）
/// 限定函数参数为函数
fn demo_1() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    // fn do_twice(f: Box<Fn(i32) -> i32>, arg: i32) -> i32 {
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 10);
    println!("the answer is {}", answer);
    let answer = do_twice(|x| x + 3, 5);
    println!("the answer is {}", answer);
}

/// 可传闭包也可以传函数
fn demo_2() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list {:?}", list_of_strings);
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list {:?}", list_of_strings);
}

/// 元组当函数用
fn demo_3() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("list_of_statues: {:?}", list_of_statuses);

    #[derive(Debug)]
    struct TurTest(i32);
    let list_of_tur: Vec<TurTest> = (0..20).map(TurTest).collect();
    println!("list_of_tur: {:?}", list_of_tur);
}

/// 返回闭包
fn demo_4() {
    // 闭包
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    // 函数
    fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
        fn closure(x: i32) -> i32 { x + 1 }
        Box::new(closure)
    }

    // 调用闭包
    let cl = returns_closure();
    println!("returns_closure: {}", (*cl)(20));

    // 调用函数
    let cl = returns_closure2();
    println!("returns_closure: {}", (*cl)(10));
}

pub fn main() {
    demo_1();
    demo_2();
    demo_3();
    demo_4();
}
