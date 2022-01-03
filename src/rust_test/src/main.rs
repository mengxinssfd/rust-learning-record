use std::fmt::Binary;

mod lifetime_test;
mod map_test;
mod match_test;
mod mod_test;
mod option_test;
mod string_test;
mod type_test;

#[derive(Debug)]
struct Rectangle {
    w: i32,
    h: i32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> i32 {
        self.w * self.h
    }
    // 关联函数
    fn square(size: i32) -> Rectangle {
        Rectangle { w: size, h: size }
    }
}
// 可以多个实现
impl Rectangle {}

#[derive(Debug)]
enum State {
    OK = 200,
    NotFound = 404,
}

#[warn(dead_code)]
#[allow(unused_variables)]
fn main() {
    assert_ne!(0.1 + 0.2, 0.3); // 0.1 + 0.2 = 0.3000000000004

    let s = String::from("hello world");
    let mut chars = s.chars();
    println!("s[0] {:?}", chars.nth(0).unwrap()); // h
    println!("s[0] {:?}", chars.nth(0).unwrap()); // e  h不能在用了
                                                  // println!("length:{}", s.len()); // 11
    assert_eq!(s.len(), 11);
    // println!("{}", s[0]); // error
    println!("slice切片:{}", &s[0..5]); // "hello"
    println!("slice切片:{}", &s[..5]); // "hello"

    // assert_eq!(&[0..5], "hello"); // error
    println!("slice切片:{}", &s[6..s.len()]); // "world"
    println!("slice切片:{}", &s[6..]); // "world"

    println!("slice切片:{}", &s[..]); // "hello world"

    println!("根据下标获取值 {:?}", s.chars().nth(0)); // Some("h")

    // println!("测试replace {}", s.replace(/test/,''))

    let s2 = String::from("你好");
    assert_eq!(s2.len(), 6);
    assert_eq!(s2.chars().count(), 2);
    // println!("{}", &s2[..1]); // error
    println!("中文切片前3字符 {}", &s2[..3]); // 你

    let s3 = "test";
    println!("{}", s3);
    println!("&str slice {}", &s3[1..2]);

    let arr = [1, 2, 3];
    println!("arr[1] {}", arr[1]); // 2
    println!("arr slice {:?}", &arr[1..3]); // [2,3]

    let v = vec![1, 2, 3];
    println!("ver[1] {}", &v[1]); // 2
    let f: i32 = match v.get(5) {
        Some(&num) => num,
        None => 5,
    };
    println!("ver[5] {}", f); // 2
    println!("ver slice {:?}", &v[1..3]); // [2,3]

    let t = (1, 2, 3);
    println!("t.0 {}", t.0); // 1

    let p = Rectangle { w: 10, h: 20 };
    println!("x:{}", p.w);

    // 解构不像js能放后面覆盖前面，在rust里面解构不能覆盖值
    let p2 = Rectangle { w: 30, ..p };
    println!("x:{},y:{}", p2.w, p2.h);
    println!("area {}", area(&p2));
    println!("area {}", p2.area());
    // 加了#[derive(Debug)]才能打印结构体
    println!("p2 {:?}", p2); // 行内
    println!("p2 {:#?}", p2); // 多行

    // let ac = area_closure(&p2);
    // println!("ac {}", ac().w)

    let sq = Rectangle::square(20);
    println!("square {:#?}", sq);
    println!("square area {}", sq.area());

    println!("enum OK {}", State::OK as i32);
    println!("enum OK {}", State::NotFound as i32);

    println!("{:?}", Some(5).unwrap());

    // let parse:dyn Binary = "10".parse().unwrap();
    // println!("parse {}", parse);

    println!("-------------mode test-------------");
    mod_test::main();
    println!("-------------mode test-------------\n");
    println!("-------------match_test-------------");
    match_test::match_test();
    println!("-------------match_test-------------\n");
    println!("-------------string_test-------------");
    string_test::string_test();
    println!("-------------string_test-------------\n");
    println!("-------------map_test-------------");
    map_test::map_test();
    println!("-------------map_test-------------");
    println!("-------------type_test-------------");
    type_test::main();
    println!("-------------type_test-------------");
    println!("-------------lifetime_test-------------");
    lifetime_test::main();
    println!("-------------lifetime_test-------------");
    println!("-------------option_test-------------");
    option_test::main();
    println!("-------------option_test-------------");
}

// 不可修改值
fn area(r: &Rectangle) -> i32 {
    // r.w = 10; // error
    r.w * r.h
}

// 可修改值
// fn area(r: &mut Rectangle) -> i32 {
//     r.w = 10;
//     r.w * r.h
// }

// 用不了，不能像js那样返回一个闭包
/*fn area_closure(r: &Rectangle) -> fn() -> &'static Rectangle {
    || {
        &r
    }
}*/
