/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut start = &0;
        let mut pool = vec![];
        let mut res = 0;

        for i in &height {
            if i > start {
                for n in pool {
                    res += start - n;
                }
                pool = vec![];
                start = i;
            } else {
                pool.push(*i);
            }
        }

        if pool.len() > 1 {
            pool.reverse();
            // 可能出现两头最大，死循环，不妨加高一下
            pool.push(*start + 1);
            res += Solution::trap(pool);
            res
        } else {
            res
        }
    }
}
// @lc code=end

