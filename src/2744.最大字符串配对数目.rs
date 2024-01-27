/*
 * @lc app=leetcode.cn id=2744 lang=rust
 *
 * [2744] 最大字符串配对数目
 */

// @lc code=start
impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut counter = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if Self::is_reverse_word_pair(&words[i], &words[j]) {
                    counter += 1;
                }
            }
        }

        counter
    }

    pub fn is_reverse_word_pair(word_a: &String, word_b: &String) -> bool {
        if word_a.len() != word_b.len() {
            return false;
        }

        let collection_a: Vec<char> = word_a.chars().collect();
        let collection_b: Vec<char> = word_b.chars().collect();

        let len = collection_a.len();

        for i in 0..len {
            if collection_a[i] != collection_b[len - i - 1] {
                return false;
            }
        }

        true
    }
}
// @lc code=end

