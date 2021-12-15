/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        for i in 1..num_rows as usize {
            let mut v = vec![];
            v.push(1);
            let mut mid = i - 1;
            let mut j = 0;
            while mid > 0 {
                v.push(res[i - 1][j] + res[i - 1][j + 1]);
                j += 1;
                mid -= 1;
            }
            v.push(1);
            res.push(v);
        }

        res
    }
}
// @lc code=end

