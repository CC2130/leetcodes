/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    }

    //pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    //    let mut res = vec![];
    //    if nums.len() < 3 {
    //        return res
    //    }

    //    let mut hs = std::collections::HashSet::new();
    //    let mut hm: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

    //    for i in 0..nums.len() {
    //        if hs.insert(nums[i]) {
    //            hm.insert(nums[i], 1);
    //        } else {
    //            hm.insert(nums[i], hm.get(&nums[i]).unwrap() + 1);
    //        }
    //    }

    //    let mut elements = hs.into_iter().collect::<Vec<_>>();
    //    elements.sort();

    //    let mut i = 0;
    //    let mut j;
    //    let mut c;
    //    let mut k;

    //    while i < elements.len() {
    //        c = *hm.get(&elements[i]).unwrap();
    //        if c > 1 {
    //            if elements[i] == 0 {
    //                if c > 2 {
    //                    res.push(vec![0, 0, 0]);
    //                }
    //            } else {
    //                k = 0 - elements[i] * 2;
    //                if k > elements[i] {
    //                    if let Some(_) = hm.get(&k) {
    //                        res.push(vec![elements[i], elements[i], k]);
    //                    }
    //                }
    //            }
    //        }
    //        j = i + 1;
    //        while j < elements.len() {
    //            k = 0 - elements[i] - elements[j];
    //            if k < elements[j] {
    //                break
    //            }
    //            if let Some(kv) = hm.get(&k) {
    //                if k == elements[j] {
    //                    if *kv == 1 {
    //                        j += 1;
    //                        continue
    //                    } else {
    //                        res.push(vec![elements[i], elements[j], elements[j]]);
    //                    }
    //                } else {
    //                    res.push(vec![elements[i], elements[j], k]);
    //                }
    //            }
    //            j += 1;
    //        }
    //        i += 1;
    //    }

    //    res
    //}
//    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
//        let mut res = vec![];
//        if nums.len() < 3 {
//            return res
//        }
//
//        nums.sort();
//
//        // 开夹
//        let mut i = 0;
//        let mut j = nums.len() - 2;
//        let mut t;
//        let mut l;
//        let mut r;
//        let mut mid;
//        while i < nums.len() - 2 {
//            j = nums.len() - 1;
//            while i + 1 < j {
//                t = nums[i] + nums[j];
//                if t < nums[i] || nums[j] < t {
//                    break
//                }
//
//                l = i + 1;
//                r = j;
//                while l < r {
//                    mid = (l + r) / 2;
//                    if t + nums[mid] == 0 {
//                        res.push(vec![nums[i], nums[mid], nums[j]]);
//                        break
//                    } else if t + nums[mid] < 0 {
//                        l = mid + 1;
//                    } else {
//                        r = mid;
//                    }
//                }
//
//                while nums[j] == nums[j - 1] {
//                    if j == i + 2 {
//                        break
//                    }
//                    j -= 1;
//                }
//                j -= 1;
//            }
//
//            while nums[i] == nums[i + 1] {
//                i += 1;
//                if i + 1 == nums.len() {
//                    break
//                }
//            }
//            i += 1;
//        }
//
//        res
//    }

    //pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    //    let mut res = vec![];
    //    if nums.len() < 3 {
    //        return res
    //    }

    //    nums.sort();
    //    let max = nums[nums.len() - 1];

    //    let mut i = 0;
    //    let mut j = i;

    //    while i < nums.len() - 2 {
    //        if nums[i] > 0 {
    //            break
    //        }
    //        j = i + 1;
    //        while j < nums.len() - 1 {
    //            let pre = nums[i] + nums[j];
    //            if pre > 0 {
    //                break
    //            }
    //            let mut l = j + 1;
    //            let mut r = nums.len();
    //            let mut mid;
    //            while l < r {
    //                mid = (l + r) / 2;
    //                if pre + nums[mid] == 0 {
    //                    res.push(vec![nums[i], nums[j], nums[mid]]);
    //                    break
    //                } else if pre + nums[mid] < 0 {
    //                    l = mid + 1;
    //                } else {
    //                    r = mid;
    //                }
    //            }
    //            while nums[j] == nums[j + 1] {
    //                j += 1;
    //                if j + 1 == nums.len() {
    //                    break
    //                }
    //            }
    //            j += 1;
    //        }
    //        while nums[i] == nums[i + 1] {
    //            i += 1;
    //            if i + 2 == nums.len() {
    //                break
    //            }
    //        }
    //        i += 1;
    //    }

    //    res
    //}
}
// @lc code=end

