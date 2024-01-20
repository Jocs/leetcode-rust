/*
 * @lc app=leetcode.cn id=747 lang=rust
 *
 * [747] 至少是其他数字两倍的最大数
 */

// @lc code=start
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut max_index = 0;

        for (i, &num) in nums.iter().enumerate() {
            if num > max {
                max = num;
                max_index = i;
            }
        }

        let mut is_double = true;

        for (i, &num) in nums.iter().enumerate() {
            if (i != max_index) && max < 2 * num {
                is_double = false;
            }
        }

        if is_double {
            max_index as i32
        } else {
            -1
        }
    }
}
// @lc code=end

