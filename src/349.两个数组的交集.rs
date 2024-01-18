/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut map = HashMap::new();
        let mut map2 = HashMap::new();

        for num in nums1 {
            map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        for num in nums2 {
            if map.contains_key(&num) && !map2.contains_key(&num) {
                result.push(num);
            }

            map2.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        result
    }
}
// @lc code=end

