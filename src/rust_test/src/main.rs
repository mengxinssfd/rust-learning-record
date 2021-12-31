fn main() {
    let s = String::from("hello world");
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
    println!("ver[1] {}", v[1]); // 2
    println!("ver slice {:?}", &v[1..3]); // [2,3]

    let t = (1, 2, 3);
    println!("t.0 {}", t.0); // 1
}
