/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1]
        } else if row_index == 1 {
            return vec![1, 1]
        } else if row_index == 2 {
            return vec![1, 2, 1]
        }
        let mut res = vec![1, 2, 1];
        for i in 3..row_index as usize + 1 {
            let len = res.len();
            res.push(1);
            let mid = (i + 1) / 2;
            for j in 0..mid as usize {
                res.push(res[j] + res[j + 1]);
            }
            if i % 2 == 0 {
                res.push(res[res.len() - 2]);
            }
            res = res[len..res.len()].to_vec();
        }

        let mut o = row_index as usize - res.len();
        while o != 0 {
            res.push(res[o]);
            o -= 1;
        }
        res.push(1);

        res
    }
}
// @lc code=end

