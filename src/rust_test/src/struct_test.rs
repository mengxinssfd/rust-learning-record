#[derive(Debug)]
pub struct Rectangle {
    pub w: i32,  // 外部引入时w需要设置为pub才能读写
    pub h: i32,
}

impl Rectangle {
    pub fn new(w: i32, h: i32) -> Rectangle {
        Rectangle { w, h }
    }
    // 方法
    pub fn area(&self) -> i32 {
        self.w * self.h
    }
    // 关联函数
    pub fn square(size: i32) -> Rectangle {
        Rectangle { w: size, h: size }
    }
}

// 可以多个实现
impl Rectangle {}

pub fn main() {
    let p = Rectangle { w: 10, h: 20 };
    println!("x:{}", p.w);

    // 解构不像js能放后面覆盖前面，在rust里面解构不能覆盖值
    let p2 = Rectangle { w: 30, ..p };
    println!("x:{},y:{}", p2.w, p2.h);
    println!("area {}", area(&p2));
    println!("area {}", p2.area());
    // 加了#[derive(Debug)]才能打印结构体
    println!("p2 {:?}", p2); // 行内
    println!("p2 {:#?}", p2); // 多行

    // let ac = area_closure(&p2);
    // println!("ac {}", ac().w)

    let sq = Rectangle::square(20);
    println!("square {:#?}", sq);
    println!("square area {}", sq.area());
}


// 不可修改值
fn area(r: &Rectangle) -> i32 {
    // r.w = 10; // error
    r.w * r.h
}

// 可修改值
// fn area(r: &mut Rectangle) -> i32 {
//     r.w = 10;
//     r.w * r.h
// }

// 用不了，不能像js那样返回一个闭包
/*fn area_closure(r: &Rectangle) -> fn() -> &'static Rectangle {
    || {
        &r
    }
}*/
