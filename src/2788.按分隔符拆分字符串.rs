/*
 * @lc app=leetcode.cn id=2788 lang=rust
 *
 * [2788] 按分隔符拆分字符串
 */

// @lc code=start
impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = Vec::new();
        for word in words {
            for item in word.split(separator) {
                if item != "" {
                    result.push(item.to_string());
                }
            }
        }

        result
    }
}
// @lc code=end

