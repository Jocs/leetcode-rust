/*
 * @lc app=leetcode.cn id=268 lang=rust
 *
 * [268] 丢失的数字
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;

        for i in 0..=len {
            if nums.contains(&i) == false {
                return i;
            }
        }

        -1
    }
}
// @lc code=end

