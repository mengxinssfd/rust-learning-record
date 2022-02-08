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
            // nums1[m + i] = *nums2.get(i)?; // 需要当前函数返回Option才有用
            nums1[m + i] = *nums2.get(i).unwrap();
        }
        nums1.sort();
    }
    /// 倒序插入
    pub fn merge_v2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m + n - 1;
        let mut m = m - 1;
        let mut n = n - 1;
        // nums1.resize(i as usize, 0);
        // i -= 1;

        // todo 想办法优化太多的as usize
        while m >= 0 || n >= 0 {
            let cur: i32;
            if m < 0 {
                cur = nums2[n as usize];
                n -= 1;
            } else if n < 0 || nums1[m as usize] > nums2[n as usize] {
                cur = nums1[m as usize];
                m -= 1;
            } else {
                cur = nums2[n as usize];
                n -= 1;
            }
            nums1[i as usize] = cur;
            i -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn foreach_test() {
        println!("遍历");
        for i in 1..6 {
            println!("1,{}", i);
        }
        println!("遍历(包含右侧)");
        for i in 1..=6 {
            println!("2,{}", i);
        }
        println!("反向遍历");
        for i in (1..=6).rev() {
            println!("3,{}", i);
        }

        println!("遍历array");
        for i in [1, 3, 5, 7, 9] {
            println!("arr, {}", i);
        }

        println!("遍历vec");
        for i in vec![1, 3, 5, 7, 9] {
            println!("vec, {}", i);
        }

        println!("10 break 5");
        'outer: for i in 0..=10 {
            println!("outer{}", i);
            for j in 0..5 {
                println!("inner{}", j);
                if j == 3 {
                    break 'outer;
                }
            }
        }

        let mut i = 0;
        let rt = loop {
            if i == 5 {
                break i;
            }
            i += 1;
        };
        println!("loop return value is {}", rt);

        // let rt = for i in 0..10 {
        //     if i == 5 {
        //         break i; //  can only break with a value inside `loop` or breakable block
        //     }
        // };
        // println!("{:?}", rt);
    }

    #[test]
    fn usize_test() {
        // 测试负数转为usize
        fn test(n: i32) -> usize {
            n as usize
        }
        assert_eq!(test(-1), 18446744073709551615);
        assert_eq!(test(-2), 18446744073709551614);
    }

    #[test]
    fn index_out_test() {
        let mut v = vec![0, 1, 2];

        // 需要重新设置size和默认值  否则报错
        v.resize(6, 0);
        v[5] = 5;
        assert_eq!(v, vec![0, 1, 2, 0, 0, 5]);

        // 保留小数位测试
        let n = format!("{:.3}", 1.123456);
        assert_eq!(n, "1.123".to_string())
    }

    #[test]
    fn merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];

        Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        Solution::merge_v2(&mut nums1, 3, &mut vec![2, 5, 6], 3);
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
                vec![1, 4, 6, 4, 1],
            ]
        );
    }

    #[test]
    fn majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }
}
