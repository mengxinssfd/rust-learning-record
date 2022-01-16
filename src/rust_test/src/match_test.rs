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
        1|2=>println!("1|2: {}",x),
        3 => println!("3: {}",x),
        _ => {}
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
