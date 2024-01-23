/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let half: i32 = nums.len() as i32 / 2;
        let mut map = HashMap::new();

        for num in nums.iter() {
            if map.contains_key(num) {
                let count = map.get_mut(num).unwrap();
                *count += 1;

                if *count > half {
                    return *num;
                }
            } else {
                map.insert(num, 1);

                if 1 > half {
                    return *num;
                }
            }
        }

        -1
    }
}
// @lc code=end

