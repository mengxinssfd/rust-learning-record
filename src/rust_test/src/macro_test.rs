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
}

pub fn main() {
    demo_1();
}
