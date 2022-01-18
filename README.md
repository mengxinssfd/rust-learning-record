# rust-learning-record
rust学习记录

## 没搞懂的
- pub use
  > 类似于js的export xx from xx;
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
    > js也不能
- 生命周期省略
  > 第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
  >
  > 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
  >
  > 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章), 那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
- 19章 unsafe，调用C,并没有导入不知道哪来的