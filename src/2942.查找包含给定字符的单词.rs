/*
 * @lc app=leetcode.cn id=2942 lang=rust
 *
 * [2942] 查找包含给定字符的单词
 */

// @lc code=start
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        for (i, word) in words.iter().enumerate() {
            if word.chars().any(|c| c == x) {
                ret.push(i as i32);
            }
        }

        ret
    }
}
// @lc code=end

