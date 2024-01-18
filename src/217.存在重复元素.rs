/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in nums {
            if map.contains_key(&num) {
                return true;
            }

            map.insert(num, 1);
        }

        false
    }
}
// @lc code=end

