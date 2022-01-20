#![allow(unused)]

/// 高级类型

// 类型别名
fn demo_1() {
    // 跟ts一样
    type Ms = i32;

    let i: i32 = 10;
    let mut ms: Ms = 20;

    ms = i;
    println!("Ms: {}", ms);
}

fn demo_2() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }
}

/// 使用type省略一位公共类型
fn demo_3() {
    use std::io::Error;
    use std::fmt;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }

    type Result2<T> = std::result::Result<T, Error>;
    pub trait Write2 {
        fn write(&mut self, buf: &[u8]) -> Result2<usize>;
        fn flush(&mut self) -> Result2<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
    }
}

/// 从不返回的 never type
fn demo_4() {
    fn bar() -> ! {
        loop {}
    }
}

pub fn main() {
    demo_1();
    demo_2();
    demo_3();
    demo_4();
}