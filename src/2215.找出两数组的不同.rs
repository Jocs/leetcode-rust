/*
 * @lc app=leetcode.cn id=2215 lang=rust
 *
 * [2215] 找出两数组的不同
 */

// @lc code=start
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();

        for num1 in nums1.iter() {
            if !nums2.contains(&num1) {
                if !vec1.contains(num1) {
                    vec1.push(*num1);
                }
            }
        }

        for num2 in nums2.iter() {
            if !nums1.contains(&num2) {
                if !vec2.contains(num2) {
                    vec2.push(*num2);
                }
            }
        }

        result.push(vec1);
        result.push(vec2);

        result
    }
}
// @lc code=end

