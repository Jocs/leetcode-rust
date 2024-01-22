/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] 存在重复元素 II
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let len = nums.len();

        for i in 0..len {
            for j in (i + 1)..cmp::min((i + k as usize + 1), len) {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end

