/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ai = a.chars().rev();
        let mut bi = b.chars().rev();
        let mut res = String::new();
        let mut add = 0;
        let from_char = | c | {
            if c == '1' {
                1
            } else {
                0
            }
        };

        loop {
            match (ai.next(), bi.next()) {
                (Some(i), Some(j)) => {
                    let v = from_char(i) + from_char(j) + add;
                    if v > 2 {
                        res.insert(0, '1');
                        add = 1;
                    } else if v == 2 {
                        res.insert(0, '0');
                        add = 1;
                    } else if v == 1 {
                        res.insert(0, '1');
                        add = 0;
                    } else {
                        res.insert(0, '0');
                        add = 0;
                    }
                },
                (Some(i), None) | (None, Some(i)) => {
                    let v = from_char(i) + add;
                    if v == 2 {
                        res.insert(0, '0');
                        add = 1;
                    } else if v == 1 {
                        res.insert(0, '1');
                        add = 0;
                    } else {
                        res.insert(0, '0');
                        add = 0;
                    }
                },
                _ => {
                    if add == 1 {
                        res.insert(0, '1');
                    }
                    break
                }
            }
        }

        res
    }
}
// @lc code=end

