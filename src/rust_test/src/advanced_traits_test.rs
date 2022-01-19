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

pub fn main() {
    demo_1();
    demo_2();
}