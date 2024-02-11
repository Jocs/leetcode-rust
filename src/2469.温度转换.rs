/*
 * @lc app=leetcode.cn id=2469 lang=rust
 *
 * [2469] 温度转换
 */

// @lc code=start
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let mut result = Vec::new();
        result.push(celsius + 273.15);
        result.push(celsius * 1.80 + 32.00);

        result
    }
}
// @lc code=end

