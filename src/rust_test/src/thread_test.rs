#![allow(unused)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn main() {
    // demo_1();
    println!("-------------------");
    demo_2();
    demo_3();
    demo_4();
    demo_5();
    demo_6();
    demo_7();
    demo_8();
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

// channel 使用channel进行线程通信
fn demo_4() {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        sender.send("hi".to_string());
    });

    // recv 是 receive 的缩写。这个方法会阻塞主线程执行直到从通道中接收一个值
    // try_recv 不会阻塞
    let received = receiver.recv().unwrap();
    println!("demo_4 received: {}", received);
}

// channel 使用channel进行线程通信 接收多个值
fn demo_5() {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in receiver {
        println!("demo_5 received: {}", i);
    }
}

// channel 使用channel进行线程通信 接收多个值 通过clone复制多个发送者
fn demo_6() {
    let (sender, receiver) = mpsc::channel();
    let sender1 = sender.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sender1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });


    for i in receiver {
        println!("demo_6 received: {}", i);
    }
}

use std::sync::Mutex;

// Mutex Mutex智能指针 主线程用锁
fn demo_7() {
    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
    main();
}

use std::sync::Arc;

// Mutex Arc智能指针 多个线程共用一个锁
fn demo_8() {
    fn main() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }
    main();
}