/*
 * @lc app=leetcode.cn id=643 lang=rust
 *
 * [643] 子数组最大平均数 I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max: f64 = 0.0;
        let mut slide_window: f64 = 0.0;

        for i in 0..nums.len() {
            let num = nums[i];

            if i < k as usize {
                slide_window += num as f64;
                max += num as f64;
            } else {
                slide_window = slide_window - nums[i - k as usize] as f64 + num as f64;
                
                if slide_window > max {
                    max = slide_window;
                }
            }
        }

        max / k as f64
    }
}
// @lc code=end

