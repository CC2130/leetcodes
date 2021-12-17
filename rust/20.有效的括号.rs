/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut buffer = vec!['f'];
        for c in s.chars() {
            match c {
                '{' | '[' | '(' => {
                    buffer.push(c);
                },
                '}' => { if buffer.pop().unwrap() != '{' { return false }},
                ']' => { if buffer.pop().unwrap() != '[' { return false }},
                ')' => { if buffer.pop().unwrap() != '(' { return false }},
                _ => { return false }
            }
        }

        buffer.len() == 1
    }
}
// @lc code=end

