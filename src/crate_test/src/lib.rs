//! # Crate_test
//!
//! `crate_test` 是一个使得特定计算更方便的
//! 工具集合

pub use art::*; // 类似于 export xx from xx;

/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crate_test::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
