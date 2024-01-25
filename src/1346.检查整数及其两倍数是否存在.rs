/*
 * @lc app=leetcode.cn id=1346 lang=rust
 *
 * [1346] 检查整数及其两倍数是否存在
 */

// @lc code=start
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let len = arr.len();

        for i in 0..len {
            for j in i + 1..len {
                if arr[i] == 2 * arr[j] || 2 * arr[i] == arr[j] {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end

