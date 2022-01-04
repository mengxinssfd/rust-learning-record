// #[cfg(test)] // 放跟源码同一个文件下才需要#[cfg(test)]，Rust推荐单元测试写到源码文件下
// mod tests {
extern crate rust_test; // rust_test是项目名

use rust_test::*;  // 只能导入src/lib.rs下的文件

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn rectangle_test() {
    let r = Rectangle::new(10, 20);
    assert_eq!(r.area(), 200);
    assert_eq!(r.w, 10);
    assert_eq!(r.w, 10);
    assert_eq!(r.h, 20);
    assert_eq!(r.h, 20);
    let s = Rectangle::square(30);
    assert_eq!(s.area(), 900);
    assert_eq!(s.w, 30);
    assert_eq!(s.h, 30);
    assert_eq!(s.h, 30);
    assert_eq!(s.h, 30);
}

#[test]
fn not_should_panic() {
    assert_eq!(panic_test(10), 10);
}

#[test]
#[should_panic]
fn should_panic() {
    panic_test(-1);
}

#[test]
#[should_panic(expected = "num should less then 100")]
fn should_panic2() {
    panic_test(100);
}

#[test]
fn result_test() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

#[test]
#[ignore] // 测试忽略
fn expensive_test() {
    // 需要运行一个小时的代码
}


fn panic_test(num: i32) -> i32 {
    if num <= 0 { panic!("num should great then 0") };
    if num >= 100 { panic!("num should less then 100") };
    num
}
/*
 命令
 1.单线程测试 cargo                test -- --test-threads=1
 2.只测试ignore                   cargo test -- --ignored
 3.显示函数输出(成功也输出print)     cargo test -- --nocapture
 4.运行单个测试 | 过滤运行多个测试    cargo test should  // 会测试not_should_panic、should_panic、should_panic三个
*/
