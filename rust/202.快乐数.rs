/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 * [202] 快乐数
 */

// @lc code=start
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut t;

        use std::collections::HashSet;
        
        let mut store = HashSet::new();
        while store.insert(n) {
            let mut res = 0;
            while n > 0 {
                t = n % 10;
                res += t * t;
                n /= 10;
            }

            if res == 1 {
                return true
            }

            n = res;
        }

        false
    }
}
// @lc code=end

