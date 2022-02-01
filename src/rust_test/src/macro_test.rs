#![allow(unused)]

fn demo_1() {
    #[macro_export]
    macro_rules! vec2 {
        // "*"类似于正则的"*"
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    println!("x,{}", $x);
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let v = vec2![1, 2, 3];
    println!("v,{:?}", v);

    let v = vec2!(1, 2, 3);
    println!("v,{:?}", v);
}

// 自定义过程宏
fn demo_2() {
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct HelloMacroTest;

    HelloMacroTest::hello_macro();
}

// 群里写的
fn demo_3() {
    macro_rules! my {
        ($($id:ident)/+) => {
            vec![1]
        };
        ($id:ident $(+ $id2:ident)+) => {
            vec![2]
        };
        ($id:ident $(/ $id2:ident)+; $($tail:tt)+) => {
            {let mut x = vec![1];x.append(&mut my!($($tail)+));x}
        };
        ($id:ident $(+ $id2:ident)+; $($tail:tt)+) => {
            {let mut x = vec![2];x.append(&mut my!($($tail)+));x}
        };
    }

    println!("{:?}", my!(a / b));
    println!("{:?}", my!(a / b / c));
    println!("{:?}", my!(a + b));
    println!("{:?}", my!(a + b + c));
    println!("{:?}", my!(a/b;a+b+c));
    println!("{:?}", my!(a+b;a/b/c));
    println!("{:?}", my!(a+b;a/b/c;c+d));
    println!("{:?}", my!(a+b;a/b/c;c+d;c/d));
}

pub fn main() {
    demo_1();
    // let v = vec2![1, 2, 3];
    // println!("v,{:?}", v);

    demo_2();
    demo_3();
}
