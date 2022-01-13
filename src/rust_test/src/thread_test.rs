#![allow(unused)]

use std::thread;
use std::time::Duration;

pub fn main() {
    // demo_1();
    println!("-------------------");
    demo_2();
    demo_3();
}

// 主线程执行完毕后不会等待子线程
fn demo_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
// 使用join等待子线程执行结束
fn demo_2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("demo_2 hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 在此join都话会执行完子线程后再执行join后面都代码
    // handle.join().unwrap();

    for i in 1..5 {
        println!("demo_2 hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
// 闭包使用move获取变量所有权
fn demo_3() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
