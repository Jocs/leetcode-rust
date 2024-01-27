/*
 * @lc app=leetcode.cn id=1512 lang=rust
 *
 * [1512] 好数对的数目
 */

// @lc code=start
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    counter += 1;
                }
            }
        }

        counter
    }
}
// @lc code=end

