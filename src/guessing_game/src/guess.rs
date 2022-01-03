pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        let g = Guess { value };
        // g.test_fn();
        g
    }
    pub fn value(&self) -> i32 {
        self.test_fn();
        self.value
    }
    fn test_fn(&self) {
        println!("{}", self.value);
    }
}
