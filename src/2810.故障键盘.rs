/*
 * @lc app=leetcode.cn id=2810 lang=rust
 *
 * [2810] 故障键盘
 */

// @lc code=start
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut ret: Vec<char> = Vec::new();

        for c in s.chars() {
            if c != 'i' {
                ret.push(c);
            } else {
                ret.reverse();
            }
        }

        ret.iter().collect()
    }
}
// @lc code=end

