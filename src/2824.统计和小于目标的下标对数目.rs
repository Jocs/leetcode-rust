/*
 * @lc app=leetcode.cn id=2824 lang=rust
 *
 * [2824] 统计和小于目标的下标对数目
 */

// @lc code=start
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut counter = 0;

        for i in 0..len {
            for j in i + 1..len {
                if nums[i] + nums[j] < target {
                    counter += 1;
                }
            }
        }

        counter
    }
}
// @lc code=end

