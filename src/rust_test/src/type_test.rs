#![allow(unused)]

use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

// 为实现了Display的实例实现show方法
impl<T: Display> Point<T> {
    fn show(&self) {
        println!("x:{}, y:{}", self.x, self.y);
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 这段代码意味着 Point<f32> 类型会有一个方法distance_from_origin，而其他T不是f32类型的 Point<T> 实例则没有定义此方法。
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Test {
    value: i32,
}

impl Test {
    pub fn new(value: i32) -> Test {
        Test { value }
    }
}

// trait 类似接口（interface）
pub trait TestTrait {
    fn show_value(&self) -> i32;
    fn show(&self) {
        println!("trait show test value:{}", self.show_value());
    }
}

pub trait TestTrait2 {
    fn show_value2(&self) -> i32;
    fn show2(&self) {
        println!("trait2 show test value:{}", self.show_value2());
    }
}

// 实现trait
impl TestTrait for Test {
    fn show_value(&self) -> i32 {
        self.value
    }

    // 会覆盖TestTrait中的show函数
    fn show(&self) {
        // TestTrait::show(self);
        // 无法从相同方法的重载实现中调用默认方法
        println!("impl show test value:{}", self.value);
    }
}

impl TestTrait2 for Test {
    fn show_value2(&self) -> i32 {
        self.value
    }
}

// 限制参数必须实现TestTrait 方法1：impl TestTrait
fn call_impl_test(ins: &impl TestTrait) {
    // &(impl TestTrait)
    println!("\ncall_impl_test start");
    ins.show();
    println!("call_impl_test end\n");
}

// 限制参数必须实现TestTrait 方法2：T: TestTrait
fn call_impl_test2<T: TestTrait>(ins: &T) {
    println!("call_impl_test2 start");
    ins.show();
    println!("call_impl_test2 end\n");
}

// 限制参数必须实现TestTrait和TestTrait2  方法1：impl TestTrait + TestTrait2
fn call_impl_test3(ins: &(impl TestTrait + TestTrait2)) {
    println!("call_impl_test3 start");
    ins.show2();
    println!("call_impl_test3 end\n");
}

// 限制参数必须实现TestTrait和TestTrait2  方法1：impl TestTrait
fn call_impl_test4<T: TestTrait + TestTrait2>(ins: &T) {
    println!("call_impl_test4 start");
    ins.show2();
    println!("call_impl_test4 end\n");
}

// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

// 限制参数必须实现TestTrait和TestTrait3  方法1：impl TestTrait
fn call_impl_test5<T>(ins: &T)
    where
        T: TestTrait + TestTrait2,
{
    println!("call_impl_test5 start");
    ins.show2();
    println!("call_impl_test5 end\n");
}

// 找最大的数  https://rust.bootcss.com/ch10-02-traits.html
// 限制实现了比较和复制trait的
/*fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// 优化：另一种 largest 的实现方式是返回在 slice 中 T 值的引用。
// 如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，
// 我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配。
// 尝试自己实现这种替代解决方式吧！
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &(list[0]);
    // let &a = largest; // 把largest转为值 但是必须实现Copy
    // println!("a：{}", a); // 必须实现Display

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    &largest
}


// 测试传None值
fn type_test<T>(v: Option<T>) -> T {
    match v {
        Some(x) => x,
        None => panic!("12123")
    }
}

pub fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    p.show();

    let p2 = Point { x: 0.5, y: 1.5 };
    println!("p2.x distance from p2.y = {}", p2.distance_from_origin());

    let test = Test::new(10);
    test.show();
    call_impl_test(&test);
    call_impl_test2(&test);
    call_impl_test3(&test);
    call_impl_test4(&test);
    // call_impl_test5(&p2); // error 编译器报错，编辑器不会报错
    call_impl_test5(&test); // error

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    // 测试传None值  单独的None不行
    // println!("{:?}", type_test(<Option<()>>::None));
}
