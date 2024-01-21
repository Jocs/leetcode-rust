/*
 * @lc app=leetcode.cn id=605 lang=rust
 *
 * [605] 种花问题
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut counter = 0;
        let mut map = HashMap::new();

        if flowerbed.len() == 0 {
            return counter >= n;
        }

        if flowerbed.len() == 1 {
            if flowerbed[0] == 0 {
                counter += 1;
            }
            return counter >= n;
        }

        for i in 0..flowerbed.len() {
            let cur_num = flowerbed[i];
            map.insert(i, cur_num);

            if i == 0 {
                let next_num = flowerbed[i + 1];
                if cur_num == 0 && next_num == 0 {
                    counter += 1;
                    map.insert(i, 1);
                }
            } else if i == flowerbed.len() - 1 {
                let prev_num = map.get(&(i - 1)).unwrap();
                if *prev_num == 0 && cur_num == 0 {
                    counter += 1;
                    map.insert(i, 1);
                }
            } else {
                let prev_num = map.get(&(i - 1)).unwrap();
                let next_num = flowerbed[i + 1];
                if *prev_num == 0 && cur_num == 0 && next_num == 0 {
                    counter += 1;
                    map.insert(i, 1);
                }
            }


        }

        counter >= n
    }
}
// @lc code=end

