#![allow(unused)]

pub fn match_test() {
    // let some_u8_value = 0u8;
    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ 通配符
        _ => (),
    }

    let c = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", c);

    // if let
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count {}", count);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_match() {
        fn test<'a>(some_u8_value: u8) -> &'a str {
            match some_u8_value {
                1 => "one",
                3 => "three",
                5 => "five",
                7 => "seven",
                // _ 通配符
                _ => "other",
            }
        }

        assert_eq!(test(5u8), "five");
        assert_eq!(test(8u8), "other");
        assert_eq!(test(2u8), "other");
    }

    #[test]
    fn test_if_let() {
        fn test(some: Option<i32>) -> i32 {
            // if let可以解构option值
            if let Some(a) = some {
                a * 10
            } else {
                1
            }
        }

        assert_eq!(test(Some(10)), 100);
    }
}
