/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();

        for char in s.chars() {
            *map.entry(char).or_insert(0) += 1;
        }

        for char in t.chars() {
            if !map.contains_key(&char) {
                return false;
            }

            map.entry(char).and_modify(|c| *c -= 1);
        }

        for value in map.values() {
            if *value != 0 {
                return false;
            }
        }

        true
    }
}
// @lc code=end

