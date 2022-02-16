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
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);

        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
