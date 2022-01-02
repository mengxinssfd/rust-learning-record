use std::collections::HashMap;

pub fn map_test() {
    let a = "a";
    let one = 1;
    let mut map = HashMap::new();
    // map.insert(String::from("a"), 1);
    map.insert(a, one);

    // 上面两种类型 都用下面的方式取
    println!("map.get('a') {:?}", map.get("a")); // Some(1)
    println!("map.get('a') {:?}", map.get("a").unwrap()); // 1
    println!("map {:?}", map);
    println!("a {:?}", a); // 还能用
    println!("one {:?}", one); // 还能用
    map.insert(a,2); // 更新值
    println!("map.get('a') {:?}", map.get("a")); // Some(2)
    map.entry(a).or_insert(3); // 只在键没有对应值时插入
    println!("map.get('a') {:?}", map.get("a")); // Some(2)
    map.entry("b").or_insert(3); // 只在键没有对应值时插入
    println!("map.get('b') {:?}", map.get("b")); // Some(3)

    let mut map2 = HashMap::new();
    let v = String::from("v");
    let k = String::from("k");
    map2.insert(k, v);
    println!("map2 {:?}", map2);
    // println!("k {}", k); // k不能用了
    // println!("v {}", v); // v不能用了

    // vec转hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("vec => hashmap {:?}", scores); // vec => hashmap {"Blue": 10, "Yellow": 50}
    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);  // {"wonderful": 1, "hello": 1, "world": 2}
}
