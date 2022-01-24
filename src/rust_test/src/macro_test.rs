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

pub fn main() {
    demo_1();
    // let v = vec2![1, 2, 3];
    // println!("v,{:?}", v);

    demo_2();
}
