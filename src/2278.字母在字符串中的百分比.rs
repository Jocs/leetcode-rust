/*
 * @lc app=leetcode.cn id=2278 lang=rust
 *
 * [2278] 字母在字符串中的百分比
 */

// @lc code=start
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let len = s.len();
        let mut counter = 0;

        for ch in s.chars() {
            if ch == letter {
                counter += 1;
            }
        }

        counter * 100 / len as i32
    }
}
// @lc code=end

