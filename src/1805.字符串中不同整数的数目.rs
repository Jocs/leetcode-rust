/*
 * @lc app=leetcode.cn id=1805 lang=rust
 *
 * [1805] 字符串中不同整数的数目
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut set: HashSet<&str> = HashSet::new();
        let mut new_word = String::new();

        for char in word.chars() {
            if char.is_ascii_digit() {
                new_word.push(char);
            } else {
                new_word.push(' ');
            }
        }

        for word in new_word.split_whitespace() {
            if word.len() == 0 {
                continue;
            }

            set.insert(word.trim_start_matches('0'));
        }

        set.len() as i32
    }
}
// @lc code=end

