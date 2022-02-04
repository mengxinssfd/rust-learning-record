use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        /*  let mut i = 0;
        s.chars().for_each(|c| {
             if map.contains_key(&c) {
                 map.insert(c, -1);
             } else {
                 map.insert(c, i);
             }
             i += 1;
         });

         // println!("{:?}", map);

         for x in s.chars() {
             match map.get(&x) {
                 Some(&v) if v != -1 => return v,
                 _ => {}
             }
         }
         */

        /*
        for (i,c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                map.insert(c, -1);
            } else {
                map.insert(c, i as i32);
            }
        };

        for x in s.chars() {
            if map[&x] != -1 {
                return map[&x];
            }
        }
         */
        s.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });

        println!("{:?}", map);
        for (i, c) in s.chars().enumerate() {
            if map[&c] == 1 {
                return i as i32;
            }
        }
        -1
    }

    pub fn reverse_prefix(word: String, ch: char) -> String {
        // let i = word.chars().position(|c| c == ch);
        let i = word.find(ch);

        match i {
            None => word,
            Some(i) => {
                let prefix = (&word[..i + 1]).chars().rev().collect::<String>();
                let affix = &word[i + 1..];
                prefix + affix
            }
        }
    }

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

    /// 389. 找不同  // https://leetcode-cn.com/problems/find-the-difference/
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut arr: [i8; 26] = [0; 26];
        let a = 'a' as usize;
        s.chars().for_each(|c| arr[(c as usize) - a] += 1);
        t.chars().for_each(|c| arr[(c as usize) - a] -= 1);
        let i = arr.iter().position(|&v| v == -1).unwrap();
        (a as u8 + i as u8) as char
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }
    #[test]
    fn find_the_difference() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
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
    #[test]
    fn reverse_prefix() {
        assert_eq!(
            Solution::reverse_prefix("leetcode".to_string(), 't'),
            "teelcode".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("leetcode".to_string(), 'l'),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("leetcode".to_string(), 'h'),
            "leetcode".to_string()
        );
    }
    #[test]
    fn first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);

        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
