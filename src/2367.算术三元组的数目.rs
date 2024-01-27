/*
 * @lc app=leetcode.cn id=2367 lang=rust
 *
 * [2367] 算术三元组的数目
 */

// @lc code=start
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let len = nums.len();
        let mut counter = 0;

        for i in 0..len {
            for j in i + 1..len {
                for k in j + 1..len {
                    let num_i = nums[i];
                    let num_j = nums[j];
                    let num_k = nums[k];

                    if num_j - num_i == diff && num_k - num_j == diff {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }
}
// @lc code=end

