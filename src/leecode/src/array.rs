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
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }
}
