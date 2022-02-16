use crate::solution::Solution;

impl Solution {
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
    fn find_the_difference() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
}
