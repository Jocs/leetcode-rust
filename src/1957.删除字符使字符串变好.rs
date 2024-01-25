/*
 * @lc app=leetcode.cn id=1957 lang=rust
 *
 * [1957] 删除字符使字符串变好
 */

// @lc code=start
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        if s.len() < 3 {
            return s;
        }

        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let mut pre_pre_char = chars[0];
        let mut pre_char = chars[1];

        result.push(pre_pre_char);
        result.push(pre_char);

        for i in 2..chars.len() {
            let char = chars[i];
            if char != pre_char || pre_pre_char != char {
                result.push(char);;
            }

            pre_pre_char = pre_char;
            pre_char = char;
        }

        result
    }
}
// @lc code=end

