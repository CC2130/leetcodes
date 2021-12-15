/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = nums[0];
        nums.iter().skip(1).for_each(|&x| {
            sum = x.max(sum + x);
            max = max.max(sum);
        });

        max
    }
    //pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    //    let mut sum = nums[0];
    //    let mut start = 0;
    //    let mut end = 1;

    //    // 找到第一个非负数
    //    let mut start = 0;
    //    let mut max = nums[0];
    //    let mut all_negative = true;
    //    for i in 0..nums.len() {
    //        if nums[i] > 0 {
    //            start = i;
    //            all_negative = false;
    //            max = nums[i];
    //            break;
    //        }
    //        if nums[i] > max {
    //            max = nums[i];
    //        }
    //    }

    //    if all_negative {
    //        return max
    //    }

    //    let mut tmp = 0;
    //    while start < nums.len() {
    //        tmp += nums[start];
    //        if tmp > max {
    //            max = tmp;
    //        }
    //        if tmp < 0 {
    //            tmp = 0;
    //        }
    //        start += 1;
    //    }

    //    max
    //}
}
// @lc code=end

