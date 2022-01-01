pub fn string_test() {
    let hello: String = String::from("hello");
    let world: String = "world".to_string();
    println!("{} {}", hello, world);

    // let mut hw = hello + &world; // 注意 hello 被移动了，不能继续使用 因为没有复制值 效率会高一点
    // let mut hw = String::from(&hello[..]) + &world; // // 复制了hello
    // let mut hw = (&hello[..]).to_string() + &world; // // 复制了hello
    let mut hw = hello.clone() + &world; // // 复制了hello
    println!("{}", hw); // helloworld
    hw = format!("{} {}", hello, world); // 使用format!()，hello还能用
    println!("{} {}", hw, hello); // hello world hello

    println!("chars[3] {:?}", substr(&hw, 3, 1));
    // let char = hw.chars().skip(3).take(1).collect(); // 未声明类型 报错
    let char: String = hw.chars().skip(3).take(1).collect();
    println!("chars[3] {:?}", char);
}
fn substr(s: &str, start: usize, length: usize) -> String {
    s.chars().skip(start).take(length).collect()
}
