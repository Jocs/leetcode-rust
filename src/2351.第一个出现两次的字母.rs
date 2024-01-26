/*
 * @lc app=leetcode.cn id=2351 lang=rust
 *
 * [2351] 第一个出现两次的字母
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut set: HashSet<char> = HashSet::new();

        for c in s.chars() {
            if set.contains(&c) {
                return c;
            }

            set.insert(c);
        }

        'a'
    }
}
// @lc code=end

