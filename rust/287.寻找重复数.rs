/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let sum = ((1 + nums.len()) as f64 * nums.len() as f64 / 2) as i32;
        let mut res;
        for i in nums {
            res += i;
        }

        if res > sum 
    }
}
// @lc code=end

