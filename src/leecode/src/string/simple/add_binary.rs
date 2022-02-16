use crate::solution::Solution;

impl Solution {
    /// 67. 二进制求和
    pub fn add_binary(a: String, b: String) -> String {
        let mut len1 = (a.len() - 1) as i32;
        let mut len2 = (b.len() - 1) as i32;

        let mut carry = 0;
        let mut res = String::from("");

        let a = a.as_bytes();
        let b = b.as_bytes();

        while len1 > -1 || len2 > -1 {
            let num1 = *a.get(len1 as usize).unwrap_or(&48) - 48;
            let num2 = *b.get(len2 as usize).unwrap_or(&48) - 48;

            let mut val = num1 + num2 + carry;
            carry = (val / 2) as u8;
            val = val % 2;
            res.insert(0, (val + 48) as char);

            len1 -= 1;
            len2 -= 1;
        }

        if carry > 0 {
            res.insert(0, 49 as char);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn add_binary() {
        assert_eq!(
            Solution::add_binary("1111".to_string(), "1111".to_string()),
            "11110".to_string()
        );
        let s = "12345".to_string();
        let len = s.len() - 1;
        let mut cs = s.chars();
        println!("{:?}", cs);
        for i in 0..len {
            println!("{:?}, {:?}", cs.nth(len - i), cs);
        }

        for &b in "12345".as_bytes() {
            println!("b {}", b - 48);
        }

        println!("bytes{:?}", "12345".as_bytes());

        let ten = "10".to_string();
        let one = "1".to_string();
        assert_eq!(one + &ten, "110".to_string());
        assert_eq!(ten.chars().nth(0).unwrap(), '1');
    }
}
