/*
 * @lc app=leetcode.cn id=260 lang=rust
 *
 * [260] 只出现一次的数字 III
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let mut hs = HashSet::new();

        for i in &nums {
            if !hs.insert(i) {
                hs.remove(i);
            }
        }

        hs.iter().map(|x| **x).collect::<Vec<i32>>()

    }
}
// @lc code=end

