/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new_digists = Vec::new();
        let mut step = 0;
        let mut is_first = true;

        for digit in digits.iter().rev() {
            let mut num;
            if (is_first) {
                num = digit + step + 1;
                is_first = false;
            } else {
                num = digit + step;
            }
            

            if num > 9 {
                step = 1;

                new_digists.push(num % 10);
            } else {
                step = 0;
                new_digists.push(num);
            }
        }

        if step > 0 {
            new_digists.push(step);
        }

        new_digists.reverse();

        new_digists
    }
}
// @lc code=end

