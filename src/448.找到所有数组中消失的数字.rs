/*
 * @lc app=leetcode.cn id=448 lang=rust
 *
 * [448] 找到所有数组中消失的数字
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let len: i32 = nums.len() as i32;
        let mut map = HashMap::new();

        for num in nums.iter() {
            if map.contains_key(&num) {
                continue;
            } else {
                map.insert(num, 1);
            }
        }

        for i in 1..=len {
            if!map.contains_key(&i) {
                result.push(i);
            }
        }

        result
    }
}
// @lc code=end

