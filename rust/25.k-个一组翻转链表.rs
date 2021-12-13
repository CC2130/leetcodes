/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut tail = &mut head;
        for _ in 0..k {
            if let Some(node) = tail {
                tail = &mut node.next;
            } else {
                return head
            }
        }

        let mut others = tail.take();
        let mut tail = Solution::reverse_k_group(others, k);

        while let Some(mut header) = head.take() {
            head = header.next.take();
            header.next = tail;
            tail = Some(header);
        }

        tail
    }
}
// @lc code=end

