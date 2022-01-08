// #![allow(unused)]只能放顶部
#![allow(unused)]

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权
    // println!("{:?}", v1_iter); // error `v1_iter` moved due to this method call

    // let v1 = vec!["1", "2", "3"];
    // println!("sum {}", v1.iter().sum()); // error  cannot satisfy `_: Sum<&&str>`

    let v1: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v1.iter()); //  Map { iter: Iter([1, 2, 3]) }
    println!("{:?}", v1.iter().map(|x| x + 1)); //  Map { iter: Iter([1, 2, 3]) }
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2.iter()); // Iter([2, 3, 4])
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn main() {
    iterator_demonstration();

    filters_by_size();

    for i in Counter::new() {
        println!("Counter iter {}", i);
    }
}

