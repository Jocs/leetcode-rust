/*
 * @lc app=leetcode.cn id=1913 lang=rust
 *
 * [1913] 两个数对之间的最大乘积差
 */

// @lc code=start
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut new_nums = nums.clone();
        new_nums.sort();
    
        let len = new_nums.len();
    
        new_nums[len - 1] * new_nums[len - 2] - new_nums[0] * new_nums[1]
    }
}
// @lc code=end

