/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut l = 0;
        let mut r = x.min((i32::MAX as f64).sqrt() as i32);;
        let mut mid;

        while l < r {
            mid = (l + r) / 2;
            if mid * mid < x {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        if l * l > x {
            l - 1
        } else {
            l
        }
    }
}
// @lc code=end

