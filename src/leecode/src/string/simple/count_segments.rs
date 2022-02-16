use crate::solution::Solution;

/// 434. 字符串中的单词数 // https://leetcode-cn.com/problems/number-of-segments-in-a-string/
impl Solution {
    // 转为char
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let mut cs = s.chars();
        let mut last = None;

        while let Some(c) = cs.next() {
            if (last.is_none() || last.unwrap() == ' ') && c != ' ' {
                count += 1;
            }
            last = Some(c);
        }

        count
    }
    // 转为bytes
    pub fn count_segments2(s: String) -> i32 {
        let mut count = 0;
        let s = s.as_bytes();

        for i in 0..s.len() {
            if (i == 0 || s[i - 1] == b' ') && s[i] != b' ' {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_byte() {
        assert_eq!(b'a', 97);
    }

    #[test]
    fn count_segments() {
        assert_eq!(Solution::count_segments("Hello, my name is John".to_string()), 5);
        assert_eq!(Solution::count_segments2("Hello, my name is John".to_string()), 5);
    }
}
