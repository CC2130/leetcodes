/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return
        }
        if m == 0 {
            for i in 0..n as usize {
                nums1[i] = nums2[i];
            }
        }

        let mut r;
        let mut l;
        let mut mid;
        let mut len = m as usize;
        for i in nums2 {
            l = 0;
            r = len;
            while l < r {
                mid = (l + r) / 2;
                if *i < nums1[mid] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            nums1.pop();
            nums1.insert(r, *i);
            len += 1;
        }
    }
}
// @lc code=end

