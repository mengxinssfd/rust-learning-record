#![allow(unused)]
// 高级trait(Advanced Traits)


/// 重载运算符
fn demo_1() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Self) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 1, y: 1 };
    // println!("p1 + p2 = {:?}", (p1 + p2));
    assert_eq!(p1 + p2, Point { x: 2, y: 2 });

    // --------------
    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    println!("Millimeters + Meters = {:?}", Millimeters(10) + Meters(20));
    // error Meters未实现Add 实现Add的必须在左侧
    // println!("Millimeters + Meters = {:?}", Meters(20) + Millimeters(10));
}

/// 完全限定语法与消歧义：调用相同名称的方法
fn demo_2() {
    struct Human;
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("fly from Pilot");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("fly from Wizard");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("fly from Human");
        }
    }

    let h = Human;
    h.fly(); // fly from Human

    // 类似于js的bind call apply等临时改变this
    Pilot::fly(&h); // fly from Pilot
    Wizard::fly(&h); // fly from Wizard
}

/// 完全限定语法与消歧义：调用相同名称的方法（无self）
fn demo_3() {
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name()); // puppy
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // Spot
}

use std::fmt;
use std::fmt::Formatter;
use std::path::Display;

/// 在另一个 trait 中使用某 trait 的功能
fn demo_4() {
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}

    let p = Point { x: 10, y: 20 };

    p.outline_print();
}

/// newtype 模式用以在外部类型上实现外部 trait
fn demo_5() {
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Wrapper {}", w);
}

pub fn main() {
    demo_1();
    demo_2();
    demo_3();
    demo_4();
    demo_5();
}