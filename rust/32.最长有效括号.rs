/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 */

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut queue = vec![-1];
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                queue.push(i as i32);
            } else {
                if queue.len() > 1 {
                    queue.pop();
                    max = max.max(i as i32 - queue.last().unwrap());
                } else {
                    queue[0] = i as i32;
                }
            }
        }
        max
    }
}
// @lc code=end

