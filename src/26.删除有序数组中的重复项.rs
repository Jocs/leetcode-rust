/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut temp = Vec::new();
        let mut last_num = nums[0];

        temp.push(last_num);

        let len = nums.len();

        for i in 1..len {
            if nums[i] != last_num {
                temp.push(nums[i]);
            }

            last_num = nums[i];
        }

        let len = temp.len();

        for i in 0..len {
            nums[i] = temp[i];
        }

        len as i32
    }
}
// @lc code=end

