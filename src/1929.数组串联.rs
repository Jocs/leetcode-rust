/*
 * @lc app=leetcode.cn id=1929 lang=rust
 *
 * [1929] 数组串联
 */

// @lc code=start
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            ret.push(nums[i]);
        }

        for i in 0..nums.len() {
            ret.push(nums[i]);
        }

        ret
    }
}
// @lc code=end

