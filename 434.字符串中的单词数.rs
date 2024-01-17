/*
 * @lc app=leetcode.cn id=434 lang=rust
 *
 * [434] 字符串中的单词数
 */

// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut counter = 0;
        let mut is_in_word = false;

        for (i, c) in s.chars().enumerate() {
            if is_in_word == false && c != ' ' {
                counter += 1;
                is_in_word = true;
            }

            if is_in_word == true && c == ' ' {
                is_in_word = false;
            }
        }


        counter
    }
}
// @lc code=end

