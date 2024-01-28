/*
 * @lc app=leetcode.cn id=2956 lang=rust
 *
 * [2956] 找到两个数组中的公共元素
 */

// @lc code=start
impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut counter_1 = 0;
        let mut counter_2 = 0;

        for i in 0..nums1.len() {
            let num_1 = nums1[i];
            if nums2.contains(&num_1) {
                counter_1 += 1;
            }
        }

        for i in 0..nums2.len() {
            let num_2 = nums2[i];
            if nums1.contains(&num_2) {
                counter_2 += 1;
            }
        }

        ret.push(counter_1);
        ret.push(counter_2);

        ret
    }
}
// @lc code=end

