#![allow(unused)]
// 当我们知道某些事是可以的而 Rust 不知道的时候，就是触及不安全代码的时候了

/// 创建指向任意内存地址的裸指针
fn demo_1() {
    // 和引用一样，裸指针是可变或不可变的，分别写作 *const T 和 *mut T。
    // 这里的星号不是解引用运算符；它是类型名称的一部分。
    // 在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值。
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    // 创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // 创建了同时指向相同内存位置 num 的裸指针 *const i32 和 *mut i32。
    // 相反如果尝试创建num的不可变和可变引用，
    // 这将无法编译因为Rust的所有权规则不允许拥有可变引用的同时拥有不可变引用。
    // 通过裸指针，就能够同时创建同一地址的可变指针和不可变指针，
    // 若通过可变指针修改数据，则可能潜在造成数据竞争
}

// 调用不安全函数或方法
fn demo_2() {
    // 必须在unsafe块里调用
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
}

use std::slice;

// 调用不安全函数或方法
fn demo_3() {
    fn use_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    use_split_at_mut();
    fn impl_split_at_mut() {
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            assert!(mid <= len);
            let ptr = slice.as_mut_ptr(); // 使用 as_mut_ptr 方法访问 slice 的裸指针

            // 我们保持索引 mid 位于 slice 中的断言。
            // 接着是不安全代码：slice::from_raw_parts_mut
            // 函数获取一个裸指针和一个长度来创建一个 slice。
            // 这里使用此函数从 ptr 中创建了一个有 mid 个项的 slice。
            // 之后在 ptr 上调用 offset 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针，
            // 使用这个裸指针并以 mid 之后项的数量为长度创建一个 slice。

            // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，
            // 并必须确信这个指针是有效的。裸指针上的 offset 方法也是不安全的，
            // 因为其必须确信此地址偏移量也是有效的指针。
            // 因此必须将 slice::from_raw_parts_mut 和 offset 放入 unsafe 块中以便能调用它们。
            // 通过观察代码，和增加 mid 必然小于等于 len 的断言，
            // 我们可以说 unsafe 块中所有的裸指针将是有效的 slice 中数据的指针。
            // 这是一个可以接受的 unsafe 的恰当用法。

            // 注意无需将 split_at_mut 函数的结果标记为 unsafe，并可以在安全 Rust 中调用此函数。
            // 我们创建了一个不安全代码的安全抽象，其代码以一种安全的方式使用了 unsafe 代码，
            // 因为其只从这个函数访问的数据中创建了有效的指针。
            unsafe {
                (slice::from_raw_parts_mut(ptr, mid),
                 slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
            }
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    impl_split_at_mut();

    /* let address = 0x01234usize;
     let r = address as *mut i32;

     let slice: &[i32] = unsafe {
         slice::from_raw_parts_mut(r, 10000) // 运行时错误，会强制退出
     };
     println!("slice {:?}", slice);*/
}

// 使用 extern 函数调用外部代码
fn demo_4() {
    // 有时你的 Rust 代码可能需要与其他语言编写的代码交互。
    // 为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）。
    // 外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数。

    // 在 extern "C" 块中，列出了我们希望能够调用的另一个语言中的外部函数的签名和名称。
    // "C" 部分定义了外部函数所使用的 应用程序接口（application binary interface，ABI） —— ABI 定义了如何在汇编语言层面调用此函数。
    // "C" ABI 是最常见的，并遵循 C 编程语言的 ABI。
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // 有点类似于ts调用其他库时写的types
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 其它语言调用 Rust 函数
fn demo_5() {
    // 也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口。
    // 不同于 extern 块，就在 fn 关键字之前增加 extern 关键字并指定所用到的 ABI。
    // 还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称。
    // Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时，
    // 这会增加用于其他编译过程的额外信息，不过会使其名称更难以阅读。
    // 每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，
    // 所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name mangling。

    // 在如下的例子中，一旦其编译为动态库并从 C 语言中链接，call_from_c 函数就能够在 C 代码中访问：
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // extern 的使用无需 unsafe。
}

pub static HELLO_WORLD: &str = "hello, world";
static mut COUNTER: u32 = 1;

// 修改静态变量
fn demo_6() {
    println!("name is: {}", HELLO_WORLD);

    fn add_to_count(inc: u32) {
        unsafe { COUNTER += inc; }
    }

    add_to_count(20);
    unsafe { println!("COUNTER: {}", COUNTER); }
}

// 实现不安全 trait
fn demo_7() {
    unsafe trait Foo {
        fn test(&self);
    }
    unsafe impl Foo for i32 {
        fn test(&self) {
            println!("unsafe test value: {}", self);
        }
    }

    let num = 10;
    num.test();
}

pub fn main() {
    demo_1();
    demo_2();
    demo_3();
    demo_4();
    demo_6();
    demo_7();
}