/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        let mut cur_len = 1;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j] > nums[j - 1] {
                    cur_len += 1;

                    if cur_len > max_len {
                        max_len = cur_len;
                    }
                } else {
                    break;
                }
            }

            cur_len = 1;
        }

        max_len
    }
}
// @lc code=end
