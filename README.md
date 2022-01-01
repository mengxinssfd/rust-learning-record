# rust-learning-record
rust学习记录

## 没搞懂的
- pub use
- ```rust
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first);
  ``` 
  这段在js中很常见的代码在rust中行不通，暂时不知道怎么让它行得通
- `vec`获取元素超出时不能使用`if let`代替`match`
- 似乎不能获取字符串中的某个字符
  > String.chars().nth()
  - 就算找到了，似乎没办法替换它