use crate::solution::Solution;

impl Solution {
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
}

#[cfg(test)]
mod test {
    use super::Solution;
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
}
