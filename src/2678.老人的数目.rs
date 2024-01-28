/*
 * @lc app=leetcode.cn id=2678 lang=rust
 *
 * [2678] 老人的数目
 */

// @lc code=start
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut counter = 0;

        for detail in details.iter() {
            if Self::get_age(&detail) > 60 {
                counter += 1;
            }
        }
        
        counter
    }

    fn get_age(detail: &String) -> i32 {
        let mut is_begin = false;
        let mut s = String::new();
        let mut counter = 0;

        for ch in detail.chars() {
            if !is_begin && ch.is_ascii_digit() {
                continue;
            }

            if !is_begin && ch.is_ascii_alphabetic() {
                is_begin = true;
                continue;
            }

            if is_begin && ch.is_ascii_digit() && counter < 2 {
                s.push(ch);
                counter += 1;
            }
        }

        s.parse::<i32>().unwrap()
    }
}
// @lc code=end

