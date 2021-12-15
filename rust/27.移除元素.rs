/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len = nums.len();
        let mut i = 0;

        while i < len {
            if nums[i] == val {
                nums.remove(i);
                len -= 1;
            } else {
                i += 1;
            }
        }

        len as i32
    }
}
// @lc code=end

