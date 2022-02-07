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

    /// 88. 合并两个有序数组  // https://leetcode-cn.com/problems/merge-sorted-array/
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        for i in 0..n {
            // nums1[m + i] = nums2[i]; // get(i).unwrap()比[i]要省空间
            nums1[m + i] = *nums2.get(i).unwrap();
        }
        nums1.sort();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];

        Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
    #[test]
    fn generate() {
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
