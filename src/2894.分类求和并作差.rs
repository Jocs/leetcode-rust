/*
 * @lc app=leetcode.cn id=2894 lang=rust
 *
 * [2894] 分类求和并作差
 */

// @lc code=start
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut num1 = 0;
        let mut num2 = 0;

        for i in 0..=n {
            if i as i32 % m == 0 {
                num2 += i as i32;
            } else {
                num1 += i as i32;
            }
        }

        num1 - num2
    }
}
// @lc code=end

