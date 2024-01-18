/*
 * @lc app=leetcode.cn id=520 lang=rust
 *
 * [520] 检测大写字母
 */

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word == word.to_ascii_uppercase() {
            return true;
        }

        if word == word.to_ascii_lowercase() {
            return true;
        }

        if word == word[0..1].to_ascii_uppercase() + &word[1..].to_ascii_lowercase() {
            return true;
        }

        return false;
    }
}
// @lc code=end

