/*
 * @lc app=leetcode.cn id=1688 lang=rust
 *
 * [1688] 比赛中的配对次数
 */

// @lc code=start
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut counter = 0;
        let mut remain = n;

        while remain > 1 {
            if remain % 2 == 0 {
                counter += remain / 2;
                remain = remain / 2;
            } else {
                counter += (remain - 1) / 2;
                remain = (remain - 1) / 2 + 1;
            }
        }

        counter
    }
}
// @lc code=end

