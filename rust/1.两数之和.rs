/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut buffer = HashMap::new();

        for i in 0..nums.len() {
            if let Some(n) = buffer.get(&(target-nums[i])) {
                return vec![*n as i32, i as i32];
            }
            buffer.insert(nums[i], i);
        }

        panic!("????");
    }
}
// @lc code=end

