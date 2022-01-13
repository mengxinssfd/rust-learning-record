#![allow(unused)]

use std::mem;
use std::ops::Deref;

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
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        }; // 后
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        }; // 先被回收
        println!("CustomSmartPointers created.");
    }
    println!("CustomSmartPointers destroyed.");

    // 手动drop
    {
        let c = CustomSmartPointer {
            data: String::from("手动drop"),
        }; // 先被回收
        mem::drop(c);
        let d = CustomSmartPointer {
            data: String::from("自动drop"),
        }; // 后
    }

    // 共用一个智能指针
    /* let a = Cons(5,
                 Box::new(Cons(10,
                               Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // error  */
    // 引用计数智能指针
    use_rc();

    use_rc_and_ref_cell();

    use_ref_loop();
}

// 引用计数智能指针 Rc
fn use_rc() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use std::rc::Rc;
    use List::{Cons, Nil};

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

// RefCell 内部可变性
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::mem;
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);

        // 不能随意设置变量,同一时刻只能存在一个borrow
        limit_tracker.set_value(80);
        let br = mock_messenger.sent_messages.borrow();
        assert_eq!(br.len(), 3);
        mem::drop(br);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 4); // 不使用drop的话 error: already borrowed
    }
}

// 类似于js的一个对象到处传，然后改了对象中的某个值，其他依赖了该对象的地方全都改变了
// RefCell<T> 只能用于单线程场景。如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误
fn use_rc_and_ref_cell(){
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// - Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// - Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
// - 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

// 引用循环
fn use_ref_loop(){
    use List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // 下一行栈溢出
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    main();
}

// 弱引用：避免引用循环
fn use_weak(){
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    main();
}