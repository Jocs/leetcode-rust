/*
 * @lc app=leetcode.cn id=1137 lang=rust
 *
 * [1137] 第 N 个泰波那契数
 */

// @lc code=start
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;

        if n == 0 {
            return t0;
        }

        if n == 1 {
            return t1;
        }

        if n == 2 {
            return t2;
        }

        for i in 3..=n {
            let temp = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = temp;
        }

        t2
    }
}
// @lc code=end

