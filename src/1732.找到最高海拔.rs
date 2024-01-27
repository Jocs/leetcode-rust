/*
 * @lc app=leetcode.cn id=1732 lang=rust
 *
 * [1732] 找到最高海拔
 */

// @lc code=start
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut sum = 0;

        for i in 0..gain.len() {
            sum += gain[i];

            if sum > ret {
                ret = sum;
            }
        }

        ret
    }
}
// @lc code=end

