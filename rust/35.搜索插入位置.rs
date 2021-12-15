/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        let mut mid;

        while l < r {
            mid = (l + r) / 2;
            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        r as i32
    }
}
// @lc code=end

