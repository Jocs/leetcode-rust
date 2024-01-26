/*
 * @lc app=leetcode.cn id=1684 lang=rust
 *
 * [1684] 统计一致字符串的数目
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut counter = 0;

        for word in words {
            if Self::only_contain(word, &allowed) {
                counter += 1;
            }
        }

        counter
    }

    fn only_contain(word: String, allowed: &String) -> bool {
        let mut set: HashSet<char> = HashSet::new();

        for c in allowed.chars() {
            set.insert(c);
        }

        let len = set.len();

        for c in word.chars() {
            set.insert(c);
        }

        let len2 = set.len();

        len == len2
    }
}
// @lc code=end

