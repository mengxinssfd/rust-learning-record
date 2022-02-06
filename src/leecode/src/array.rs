use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        nums.iter().for_each(|&n| {
            map.insert(n, map.get(&n).unwrap_or(&0) + 1);
        });
        let half = nums.len() / 2;
        *map.iter().find(|(_, &v)| v > half).unwrap().0 as i32
    }

    /// 杨辉三角  // https://leetcode-cn.com/problems/pascals-triangle/
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        for i in 2..=num_rows as usize {
            // let last_row = res.get(i as usize - 2).unwrap();
            let last_row = res.last().unwrap();
            let mut row = vec![1];
            for j in 1..i - 1 {
                row.push(*last_row.get(j - 1).unwrap() + last_row.get(j).unwrap());
            }
            row.push(1);
            res.push(row);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn generate() {
        for i in 1..6 {
            println!("1,{}", i);
        }
        for i in 1..=6 {
            println!("2,{}", i);
        }
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }
}
