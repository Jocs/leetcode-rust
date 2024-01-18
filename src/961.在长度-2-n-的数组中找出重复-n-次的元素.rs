/*
 * @lc app=leetcode.cn id=961 lang=rust
 *
 * [961] 在长度 2N 的数组中找出重复 N 次的元素
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut result: i32 = -1;

        for &num in &nums {
            let count = map.entry(num).or_insert(0);

            if *count >= 1 {
                result = num;
            }

            *count += 1;
        }

        result
    }
}
// @lc code=end

