/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let map = ['I', 'V', 'X', 'L', 'C', 'D', 'M']
            .iter()
            .zip([1, 5, 10, 50, 100, 500, 1000])
            .collect::<HashMap<_, _>>();

        s.chars().rev().fold(0, |sum, c| {
            sum + if sum > map[&c] * 4 { -map[&c] } else { map[&c] }
        })
    }
}
// @lc code=end

