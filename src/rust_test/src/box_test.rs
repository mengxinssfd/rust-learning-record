#![allow(unused)]

use std::ops::Deref;
use std::mem;

pub fn main() {
    //  递归类型，Rust无法计算List大小
    /* enum List {
        Cons(i32, List),
        Nil,
    }*/

    // error: recursive type has infinite size
    // let e = Cons(1, Cons(2, Cons(3, Nil)));
    // println!("{:?}", e);

    // 智能指针Box装箱
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let e = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", e);

    // 解引用
    let x = 1;
    let boxed_x = Box::new(x);
    println!("{},{}", boxed_x, *boxed_x); // 1,1
    assert_eq!(x, *boxed_x);

    // 自定义智能指针
    /*
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // type `MyBox<{integer}>` cannot be dereferenced
    assert_eq!(5, *y); //  error rust不知道MyBox如何解引用
    */

    struct MyBox<T>(T);
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 解引用强制多态
    fn hello(str: &str) {
        println!("hello {}", str);
    }
    let s = String::from("test");
    hello(&s);
    let s = MyBox::new(s);
    hello(&s);
    // 等价于下面这行，rust已经作了处理，不需要手动处理
    hello(&(*s)[..]);

    // Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
    // - 当 T: Deref<Target=U> 时从 &T 到 &U。
    // - 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // - 当 T: Deref<Target=U> 时从 &mut T 到 &U。


    // drop
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    {
        let c = CustomSmartPointer { data: String::from("my stuff") }; // 后
        let d = CustomSmartPointer { data: String::from("other stuff") }; // 先被回收
        println!("CustomSmartPointers created.");
    }
    println!("CustomSmartPointers destroyed.");

    // 手动drop
    {
        let c = CustomSmartPointer { data: String::from("手动drop") }; // 先被回收
        mem::drop(c);
        let d = CustomSmartPointer { data: String::from("自动drop") }; // 后
    }


    // 共用一个智能指针
    /* let a = Cons(5,
                 Box::new(Cons(10,
                               Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // error  */
    // 引用计数智能指针
    use_rc();
}

// 引用计数智能指针
fn use_rc() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));


    // print
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}