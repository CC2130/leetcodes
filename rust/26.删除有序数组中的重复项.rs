/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}
// @lc code=end


    //pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //    fn insert_left(nums: &mut Vec<i32>, n: i32) -> usize {
    //        let mut l = 0;
    //        let mut r = nums.len();
    //        let mut mid;

    //        while l < r {
    //            mid = (l + r) / 2;
    //            if n < nums[mid] {
    //                r = mid;
    //            } else {
    //                l = mid + 1;
    //            }
    //        }

    //        r
    //    }

    //    let mut i = 0;
    //    let mut len = nums.len();

    //    while i < len {
    //        let k = insert_left(nums, nums[i]);
    //        if k != i + 1 {
    //            for j in i + 1..k {
    //                nums.remove(i);
    //            }
    //            len -= k - 1 - i;
    //        }
    //        i += 1;
    //    }

    //    len as i32
    //}
    //pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //    use std::collections::HashSet;

    //    let mut store = HashSet::new();

    //    let mut i = 0;
    //    let mut len = nums.len();

    //    while i < len {
    //        if store.insert(nums[i]) {
    //            i += 1;
    //        } else {
    //            nums.remove(i);
    //            len -= 1;
    //        }
    //    }

    //    len as i32
    //}