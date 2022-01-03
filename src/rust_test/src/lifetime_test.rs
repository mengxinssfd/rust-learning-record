// 不会改变生命周期，但是会拒绝不符合的参数  该函数要求x，y 和return必须都还在
// 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
// 一旦他们形成了某种关联，
// Rust就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    //  报错string2活得不够久
    /*let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);*/
    // 跟下面一行一样的错误
    /*{
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }*/


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:#?}", i);


    // 报错
    /*let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        i = ImportantExcerpt { part: first_sentence };
    }
    println!("{:#?}", i);*/
}
