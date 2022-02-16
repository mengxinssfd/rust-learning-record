use crate::solution::Solution;

/// 434. 字符串中的单词数 // https://leetcode-cn.com/problems/number-of-segments-in-a-string/
impl Solution {
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
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn majority_element() {
        assert_eq!(Solution::count_segments("Hello, my name is John".to_string()), 5);
    }
}
