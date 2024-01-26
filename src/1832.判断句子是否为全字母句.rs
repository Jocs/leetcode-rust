/*
 * @lc app=leetcode.cn id=1832 lang=rust
 *
 * [1832] 判断句子是否为全字母句
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = HashSet::new();
        for c in sentence.chars() {
            if c.is_ascii_alphabetic() {
                set.insert(c);
            }
            if set.len() == 26 {
                return true;
            }
        }

        set.len() == 26
    }
}
// @lc code=end

