/*
 * @lc app=leetcode.cn id=387 lang=rust
 *
 * [387] 字符串中的第一个唯一字符
 */

// @lc code=start
use std::collections::HashMap;

struct CharInfo {
    index: i32,
    counter: i32,
}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, CharInfo> = HashMap::new();

        for (index, c) in s.chars().enumerate() {
            let info = map.entry(c).or_insert(CharInfo {
                index: index as i32,
                counter: 0,
            });

            info.counter += 1;
        }

        let mut min_index = -1;

        for (c, info) in &map {
            if info.counter == 1 {
                if min_index > info.index || min_index == -1 {
                    min_index = info.index;
                }
            }
        }

        min_index
    }
}
// @lc code=end

