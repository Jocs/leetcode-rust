/*
 * @lc app=leetcode.cn id=905 lang=rust
 *
 * [905] 按奇偶排序数组
 */

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for num in nums.iter() {
            if num % 2 == 0 {
                result.insert(0, *num);
            } else {
                result.push(*num);
            }
        }

        result
    }
}
// @lc code=end

