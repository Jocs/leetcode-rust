/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for i in 0..nums.len() {
            if nums[i] == 0 {
                // swap non zero, if none, return.
                let mut is_swap = false;
                for j in i + 1..nums.len() {
                    if nums[j] != 0 {
                        nums.swap(i, j);
                        is_swap = true;
                        break;
                    }
                }

                if !is_swap {
                    return;
                }
            }
        }
    }
}
// @lc code=end
