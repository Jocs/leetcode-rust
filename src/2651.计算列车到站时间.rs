/*
 * @lc app=leetcode.cn id=2651 lang=rust
 *
 * [2651] 计算列车到站时间
 */

// @lc code=start
impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}
// @lc code=end

