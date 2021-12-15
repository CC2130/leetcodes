/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut tail = &mut res;
        let mut remain;
        let mut c = (list1, list2);

        loop {
            c = match c {
                (Some(mut l1), Some(mut l2)) => {
                    if l1.val < l2.val {
                        remain = l1.next.take();
                        *tail = Some(l1);
                        tail = &mut tail.as_mut().unwrap().next;
                        (remain, Some(l2))
                    } else {
                        remain = l2.next.take();
                        *tail = Some(l2);
                        tail = &mut tail.as_mut().unwrap().next;
                        (Some(l1), remain)
                    }
                },
                (Some(mut l), None) | (None, Some(mut l)) => {
                    *tail = Some(l);
                    break;
                },
                (None, None) => {
                    break;
                }
            }
        }

        res
    }
}
// @lc code=end

