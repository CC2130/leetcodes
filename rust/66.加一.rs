/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut index = digits.len() - 1;
        let mut add = true;
        while index > 0 {
            if add {
                add = false;
                if digits[index] + 1 == 10 {
                    digits[index] = 0;
                    add = true;
                } else {
                    digits[index] = digits[index] + 1;
                    return digits;
                }
                index -= 1;
            }
        }

        if add {
            if digits[0] + 1 == 10 {
                digits[0] = 0;
                digits.insert(0, 1);
            } else {
                digits[0] = digits[0] + 1;
            }
        }

        digits
    }
}
// @lc code=end

