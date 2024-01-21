/*
 * @lc app=leetcode.cn id=941 lang=rust
 *
 * [941] 有效的山脉数组
 */

// @lc code=start
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        let mut max = arr[0];
        let mut max_index = 0;

        for (i, &num) in arr.iter().enumerate() {
            if num > max {
                max = num;
                max_index = i;
            }
        }

        if max_index == 0 || max_index == arr.len() - 1 {
            return false;
        }

        for (i, &num) in arr.iter().enumerate() {
            if i < max_index {
                let next = arr[i + 1];

                if num >= next {
                    return false;
                }
            } else if i > max_index {
                let pre = arr[i - 1];
                
                if num >= pre {
                    return false;
                }
            }
        }

        true
    }
}
// @lc code=end

