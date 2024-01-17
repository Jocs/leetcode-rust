/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] 最后一个单词的长度
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let last_word = s.split_ascii_whitespace().last();

        let mut result = match last_word {
            Some(word) => word.len() as i32,
            None => 0,
        };

        result
    }
}
// @lc code=end

