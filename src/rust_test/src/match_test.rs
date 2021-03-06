#![allow(unused)]

// 可以match都类型
// 字面值
// 解构的数组、枚举、结构体或者元组
// 变量
// 通配符
// 占位符

fn demo_1() {
    // let some_u8_value = 0u8;
    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ 通配符
        _ => (),
    }

    let c = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", c);

    // if let
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count {}", count);

    // let x = Some(5);
    let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        None => println!("Matched, y is none"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 结合 if let、else if、else if let 以及 else
fn demo_2() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// while let
fn demo_3() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// for
fn demo_4() {
    let v = vec!['a', 'b', 'c'];

    for (i, char) in v.iter().enumerate() {
        println!("index: {}, item: {}", i, char);
    }
}

// let
fn demo_5() {
    let (x, y, z) = (1, 2, 3);
    println!("x {},y {},z {}", x, y, z);
    let (x, y, ..) = (1, 2, 3);
    println!("x {},y {}", x, y);
    let (x, y, ..) = (1, 2);
    println!("x {},y {}", x, y);
    // let (x, y, ..) = (1);  // error
    // println!("x {},y {}", x, y);
}

// 函数参数 类似ts的类型写法
fn demo_6() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

// 多个模式，匹配多个值
fn demo_7() {
    let x = 1;
    match x {
        1 | 2 => println!("1|2: {}", x),
        3 => println!("3: {}", x),
        _ => {}
    }
}

// 匹配命名变量
fn demo_8() {
    let x = Some(5);
    let y = 10;
    match x {
        // 只匹配50
        Some(50) => println!("Got 50"),
        // 匹配除了50和None的任何值
        Some(y) => println!("Matched, y = {:?}", y), // 内部变量y，与外部y无关
        // 匹配以上匹配值以外的任何值，包括None
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 通过 ..= 匹配值的范围  语法允许你匹配一个闭区间范围内的值
fn demo_9() {
    // 范围只允许用于数字或 char 值
    fn mc(x: i32) {
        match x {
            // 相当于 1 <= x <= 5
            // 相当于rust的 1|2|3|4|5
            1..=5 => println!("one through five {}", x),
            _ => println!("something else {}", x),
        }
    }
    fn mc_char(x: char) {
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    mc(5);
    mc(1);
    mc(3);
    mc(0);
    mc(6);

    // early ASCII letter
    mc_char('a');
    mc_char('j');
    // late ASCII letter
    mc_char('k');
    mc_char('z');
    // something else
    mc_char('Z');
    mc_char('B');
}

// 解构结构体
fn demo_10() {
    println!("-match_test-demo_10-");
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("x:{},y:{}", x, y);
    let Point { x, .. } = p;
    println!("x:{}", x);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({},{})", x, y),
    }

    println!("-match_test-demo_10-");
}

// 解构枚举
fn demo_11() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// 解构嵌套的枚举
fn demo_12() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => {}
    }
}

// 解构结构体和元组
fn demo_13() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

// 忽略模式中的值
fn demo_14() {
    fn foo(_: i32, y: i32) {
        // println!("This code only uses the y parameter: {}", x); // error `_` not allowed here
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

// 使用嵌套的 _ 忽略部分值 匹配元组
fn demo_15() {
    let mut setting_value = Some(5);
    // let new_setting_value = Some(10);
    let new_setting_value = None;

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

/// 通过在名字前以一个下划线开头来忽略未使用的变量
fn demo_16() {
    let _x = 5;
    let y = 10;

    /* let s = Some(String::from("Hello!"));

     // error
     // 我们会得到一个错误，因为 s 的值仍然会移动进 _s，并阻止我们再次使用s。
     // 然而只使用下划线本身，并不会绑定值。
     if let Some(_s) = s {
         println!("found a string");
     }

     println!("{:?}", s);*/
}

/// 用 .. 忽略剩余值
fn demo_17() {
    // 省略尾部的值
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 省略中间的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

/// 匹配守卫提供的额外条件
fn demo_18() {
    let num = Some(10);
    match num {
        Some(x) if x > 10 => println!("x > 10"),
        Some(x) => println!("x {}", x),
        _ => {}
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);


    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/// @ 绑定
fn demo_19() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // 可以取值
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        }
        // 不能取值
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // 可以取值，但没有值范围
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

// ref
fn demo_20(){
    enum A {
        M { x: u16, y: String },
        Y(String),
    }

    fn use_x_y(x: u16, y: &String) {}
    // &String可以改为&str
    // fn use_x_y(x: u16, y: &str) {}

    fn get(input: &A) { // 这行不能修改
        match input {
            /* A::M { x, y } => {
                use_x_y(*x, y); // 有办法去掉x前面的星号嘛？
            }*/
            // ref https://rust-by-example.budshome.com/scope/borrow/ref.html#ref-%E6%A8%A1%E5%BC%8F
            &A::M { x, ref y } => {
                use_x_y(x, y);
            }
            A::Y(y) => {}
        }
    }
    fn main() {

    }

}

pub fn main() {
    demo_1();
    demo_2();
    demo_3();
    demo_4();
    demo_5();
    demo_6();
    demo_7();
    demo_8();
    demo_9();
    demo_10();
    demo_11();
    demo_12();
    demo_13();
    demo_14();
    demo_15();
    demo_16();
    demo_17();
    demo_18();
    demo_19();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_match() {
        fn test<'a>(some_u8_value: u8) -> &'a str {
            match some_u8_value {
                1 => "one",
                3 => "three",
                5 => "five",
                7 => "seven",
                // _ 通配符
                _ => "other",
            }
        }

        assert_eq!(test(5u8), "five");
        assert_eq!(test(8u8), "other");
        assert_eq!(test(2u8), "other");
    }

    #[test]
    fn test_if_let() {
        fn test(some: Option<i32>) -> i32 {
            // if let可以解构option值
            if let Some(a) = some {
                a * 10
            } else {
                1
            }
        }

        assert_eq!(test(Some(10)), 100);
    }
}
