/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.into_bytes();
        let mut max = 0;
        let mut len = 0;
        let mut buffer = HashMap::new();
        for i in 0..s.len() {
            if let Some(e) = buffer.get(&s[i]) {
                if i - e <= len {
                    len = i - e;
                } else {
                    len += 1;
                }
            } else {
                len += 1;
            }
            buffer.insert(s[i], i);

            if len > max {
                max = len;
            }
        }

        max as i32
    }
}
// @lc code=end

