/*
 * @lc app=leetcode.cn id=917 lang=rust
 *
 * [917] 仅仅反转字母
 */

// @lc code=start
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut result = String::new();

        let mut chars = Vec::new();

        for char in s.chars() {
            if char.is_ascii_alphabetic() {
                chars.push(char);
            }
        }

        let mut chars_len = chars.len();

        for char in s.chars() {
            if char.is_ascii_alphabetic() {
                result.push(chars[chars_len - 1]);
                chars_len -= 1;
            } else {
                result.push(char);
            }
        }

        result
    }
}
// @lc code=end

