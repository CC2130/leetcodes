/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
const IMIN: [u8; 11] = [45, 50, 49, 52, 55, 52, 56, 51, 54, 52, 56];
const IMAX: [u8; 10] = [50, 49, 52, 55, 52, 56, 51, 54, 52, 55];

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let xs = x.to_string();
        let rxs = xs.as_bytes();
        let mut v = Vec::new();

        let mut len = rxs.len();
        while len != 0 {
            len -= 1;
            v.push(rxs[len]);
        }

        let check_return = | valid: &[u8], data: Vec<u8> | {
            if data.len() == valid.len() {
                for i in 0..data.len() {
                    if data[i] < valid[i] {
                        return String::from_utf8(data).unwrap().parse::<i32>().unwrap()
                    } else if data[i] > valid[i] {
                        return 0
                    }
                }
            }

            String::from_utf8(data).unwrap().parse::<i32>().unwrap()
        };

        if v[v.len() - 1] == 45 {
            v.pop();
            v.insert(0, 45);
            check_return(&IMIN, v)
        } else {
            check_return(&IMAX, v)
        }
    }
}
// @lc code=end

