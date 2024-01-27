/*
 * @lc app=leetcode.cn id=2710 lang=rust
 *
 * [2710] 移除字符串中的尾随零
 */

// @lc code=start
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut ret: Vec<char> = Vec::new();
        let mut is_begin = true;
        let mut chs: Vec<char> = num.chars().collect();
        chs.reverse();
    
        for i in 0..chs.len() {
            let c = chs[i];
            if c == '0' && is_begin {
                continue;
            } else {
                is_begin = false;
                ret.push(c);
            }
        }
    
        ret.reverse();
        ret.iter().collect()
    }
}
// @lc code=end

