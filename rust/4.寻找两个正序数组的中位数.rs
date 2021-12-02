/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mid_sides = if (len1 + len2) / 2 == 1 {
            ((len1 + len2) / 2, (len1 + len2) / 2)
        } else {
            ((len1 + len2) / 2 - 1, (len1 + len2) / 2)
        };

        let mut calculator = (nums1, nums2, n1, n2, 0);

    }
    //pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //    let mut nums1 = nums1;
    //    let mut nums2 = nums2;
    //    let mut merged = Vec::new();
    //    loop {
    //        match (nums1.pop(), nums2.pop()) {
    //            (Some(n1), Some(n2)) => {
    //                if n1 > n2 {
    //                    nums2.push(n2);
    //                    merged.insert(0, n1);
    //                } else {
    //                    nums1.push(n1);
    //                    merged.insert(0, n2);
    //                }
    //            },
    //            (Some(n), None) | (None, Some(n)) => {
    //                merged.insert(0, n);
    //            },
    //            (None, None) => {
    //                break
    //            }
    //        }
    //    }

    //    if merged.len() % 2 == 1 {
    //        merged[merged.len() / 2] as f64
    //    } else {
    //        (merged[merged.len() / 2 - 1] as f64 + merged[merged.len() / 2] as f64) / 2.0
    //    }
    //}
}
// @lc code=end

