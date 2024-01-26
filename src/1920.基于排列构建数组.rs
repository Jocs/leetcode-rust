/*
 * @lc app=leetcode.cn id=1920 lang=rust
 *
 * [1920] 基于排列构建数组
 */

// @lc code=start
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let len = nums.len();

        for i in 0..len {
            ret.push(nums[nums[i] as usize]);
        }
        

        ret
    }
}
// @lc code=end

