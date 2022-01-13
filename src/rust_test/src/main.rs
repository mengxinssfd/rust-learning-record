mod lifetime_test;
mod map_test;
mod match_test;
mod mod_test;
mod option_test;
mod string_test;
mod type_test;
mod struct_test;
mod closure_test;
mod iterator_test;
mod smart_pointer_test;
mod thread_test;


#[derive(Debug)]
enum State {
    OK = 200,
    NotFound = 404,
}

#[warn(dead_code)]
#[allow(unused_variables)]
fn main() {
    assert_ne!(0.1 + 0.2, 0.3); // 0.1 + 0.2 = 0.3000000000004

    let s = String::from("hello world");
    let mut chars = s.chars();
    println!("s[0] {:?}", chars.nth(0).unwrap()); // h
    println!("s[0] {:?}", chars.nth(0).unwrap()); // e  h不能在用了
    // println!("length:{}", s.len()); // 11
    assert_eq!(s.len(), 11);
    // println!("{}", s[0]); // error
    println!("slice切片:{}", &s[0..5]); // "hello"
    println!("slice切片:{}", &s[..5]); // "hello"

    // assert_eq!(&[0..5], "hello"); // error
    println!("slice切片:{}", &s[6..s.len()]); // "world"
    println!("slice切片:{}", &s[6..]); // "world"

    println!("slice切片:{}", &s[..]); // "hello world"

    println!("根据下标获取值 {:?}", s.chars().nth(0)); // Some("h")

    // println!("测试replace {}", s.replace(/test/,''))

    let s2 = String::from("你好");
    assert_eq!(s2.len(), 6);
    assert_eq!(s2.chars().count(), 2);
    // println!("{}", &s2[..1]); // error
    println!("中文切片前3字符 {}", &s2[..3]); // 你

    let s3 = "test";
    println!("{}", s3);
    println!("&str slice {}", &s3[1..2]);

    let arr = [1, 2, 3];
    println!("arr[1] {}", arr[1]); // 2
    println!("arr slice {:?}", &arr[1..3]); // [2,3]

    let v = vec![1, 2, 3];
    println!("ver[1] {}", &v[1]); // 2
    let f: i32 = match v.get(5) {
        Some(&num) => num,
        None => 5,
    };
    println!("ver[5] {}", f); // 2
    println!("ver slice {:?}", &v[1..3]); // [2,3]

    let t = (1, 2, 3);
    println!("t.0 {}", t.0); // 1


    println!("enum OK {}", State::OK as i32);
    println!("enum OK {}", State::NotFound as i32);

    println!("{:?}", Some(5).unwrap());

    // let parse:dyn Binary = "10".parse().unwrap();
    // println!("parse {}", parse);

    test_for_each();

    println!("-------------mode test-------------");
    mod_test::main();
    println!("-------------mode test-------------\n");
    println!("-------------match_test-------------");
    match_test::match_test();
    println!("-------------match_test-------------\n");
    println!("-------------string_test-------------");
    string_test::string_test();
    println!("-------------string_test-------------\n");
    println!("-------------map_test-------------");
    map_test::map_test();
    println!("-------------map_test-------------");
    println!("-------------type_test-------------");
    type_test::main();
    println!("-------------type_test-------------");
    println!("-------------lifetime_test-------------");
    lifetime_test::main();
    println!("-------------lifetime_test-------------");
    println!("-------------struct_test-------------");
    struct_test::main();
    println!("-------------struct_test-------------");
    println!("-------------closure_test-------------");
    closure_test::main();
    println!("-------------closure_test-------------");
    println!("-------------iterator_test-------------");
    iterator_test::main();
    println!("-------------iterator_test-------------");
    println!("-------------smart_pointer_test-------------");
    smart_pointer_test::main();
    println!("-------------smart_pointer_test-------------");
    println!("-------------thread_test-------------");
    thread_test::main();
    println!("-------------thread_test-------------");
    println!("-------------option_test-------------");
    option_test::main();
    println!("-------------option_test-------------");
}

fn test_for_each() {
    (0..5).for_each(|i| {
        println!("for_each {}", i);
    });
    for i in 0..5 {
        println!("for in {}", i);
        if i == 3 {
            break
        }
    }
}