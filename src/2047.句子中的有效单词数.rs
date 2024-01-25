/*
 * @lc app=leetcode.cn id=2047 lang=rust
 *
 * [2047] 句子中的有效单词数
 */

// @lc code=start
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut counter = 0;

        for word in sentence.split_ascii_whitespace() {
            let len = word.len();
            let mut need_continue = false;
            let mut _counter = 0;
            let mut p_counter = 0;
            let mut pre_char = ' ';
            let mut is_ = false;

            for (i, char) in word.chars().enumerate() {
                // has no number
                if char.is_ascii_digit() {
                    need_continue = true;
                }

                if is_ && !char.is_ascii_alphabetic() {
                    need_continue = true;
                }

                if char == '-' {
                    _counter += 1;
                    is_ = true;
                } else {
                    is_ = false;
                }

                if _counter > 1 {
                    need_continue = true;
                }
                if char == '-' && (i == 0 || i == len - 1) {
                    need_continue = true;
                }

                if char == '-' && i > 0 && i < len - 1 {
                    if !pre_char.is_ascii_alphabetic() {
                        need_continue = true;
                    }
                }

                if char == '!' || char == '.' || char == ',' {
                    p_counter += 1;

                    if i != len - 1 {
                        need_continue = true;
                    }
                }

                if p_counter > 1 {
                    need_continue = true;
                }

                pre_char = char;
            }

            if need_continue {
                continue;
            }

            counter += 1;
        }

        counter
    }
}
// @lc code=end

