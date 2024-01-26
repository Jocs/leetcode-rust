/*
 * @lc app=leetcode.cn id=2974 lang=rust
 *
 * [2974] 最小数字游戏
 */

// @lc code=start
impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        nums.sort();

        let mut temp = 0;

        for i in 0..nums.len() {
            if i % 2 == 0 {
                temp = nums[i];
            } else {
                ret.push(nums[i]);
                ret.push(temp);
            }
        }

        ret
    }
}
// @lc code=end

