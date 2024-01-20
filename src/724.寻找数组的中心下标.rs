/*
 * @lc app=leetcode.cn id=724 lang=rust
 *
 * [724] 寻找数组的中心下标
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut left: i32 = 0;

        for &num in nums.iter() {
            sum = sum + num;
        }

        for (i, &num) in nums.iter().enumerate() {
            if left == sum - num - left {
                return i as i32;
            }

            left = left + nums[i];
        }

        -1
    }
}
// @lc code=end

