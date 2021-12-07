/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 {
            return find_median_sorted_array(nums2)
        } else if nums2.len() == 0 {
            return find_median_sorted_array(nums1)
        }

        let len1 = nums1.len();
        let len2 = nums2.len();
        let mid = (len1 + len2) / 2;

        let mut l = 0;
        let mut r = len2;
        let mut index = 0;
        let mut insert_index = 0;
        let mut location = 0;
        let mut calculator = (None, 0, 0);

        while l < r {
            calculator = match calculator {
                (None, _, _) => {
                    index = (l + r) / 2;
                    insert_index = insert(&nums1, nums2[index]);
                    location = index + insert_index;
                    if location < mid {
                        l = index + 1;
                        (None, 0, insert_index)
                    } else {
                        r = index;
                        (Some(location), index, insert_index) // must got mid <= location
                    }
                },
                (Some(_location), _index, _insert_index) => {
                    if _location == mid {
                        break
                    }
                    index = (l + r) / 2;
                    insert_index = insert(&nums1, nums2[index]);
                    location = index + insert_index;

                    if location < mid {
                        l = index + 1;
                        (Some(_location), _index, _insert_index)
                    } else {
                        r = index;
                        (Some(location), index, insert_index)
                    }
                }
            };
        }

        index = calculator.1;
        insert_index = calculator.2;

        if (len1 + len2) %2 == 1 {
            if calculator.0 == None {
                return nums1[mid - len2] as f64
            } else {
                if location == mid {
                    return nums2[index] as f64
                } else {
                    return nums1[mid - index] as f64
                }
            }
        } else {
            if calculator.0 == None {
                if location + 1 == mid {
                    return (nums2[len2 - 1] as f64 + nums1[mid - len2] as f64) / 2.0
                } else {
                    return (nums1[mid - len2 - 1] as f64 + nums1[mid - len2] as f64) / 2.0
                }
            } else {
                if location == mid {
                    if insert_index > 0 {
                        if index > 0 && nums1[insert_index - 1] < nums2[index - 1] {
                            return (nums2[index - 1] as f64 + nums2[index] as f64) / 2.0
                        } else {
                            return (nums1[insert_index - 1] as f64 + nums2[index] as f64) / 2.0
                        }
                    } else {
                        return (nums2[index - 1] as f64 + nums2[index] as f64) / 2.0
                    }
                } else {
                    if index > 0 {
                        index -= 1;
                        insert_index = insert(&nums1, nums2[index]);
                        location = index + insert_index;

                        if location + 1 == mid {
                            return (nums2[index] as f64 + nums1[mid - index - 1] as f64) / 2.0
                        } else {
                            return (nums1[mid - index - 2] as f64 + nums1[mid - index -1] as f64) / 2.0
                        }
                    } else {
                        return (nums1[mid - 1] as f64 + nums1[mid] as f64) / 2.0
                    }
                }
            }
        }

        0.0
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
fn find_median_sorted_array(nums: Vec<i32>) -> f64 {
    if nums.len() % 2 == 1 {
        nums[nums.len() / 2] as f64
    } else {
        if 1 < nums.len() {
            (nums[nums.len() / 2 - 1] as f64 + nums[nums.len() / 2] as f64) / 2.0
        } else {
            nums[0] as f64
        }
    }
}

fn insert(arr: &Vec<i32>, n: i32) -> usize {
    let len = arr.len();

    let mut left = 0;
    let mut right = len;
    let mut mid;

    while left < right {
        mid = (left + right) / 2;
        if arr[mid] <= n {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
// @lc code=end

