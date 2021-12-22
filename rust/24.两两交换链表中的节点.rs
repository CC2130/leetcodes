/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        let mut other;
        let mut queue = vec![];

        'a: loop {
            for _ in 0..2 {
                if let Some(node) = tail {
                    tail = &mut node.next;
                } else {
                    break 'a;
                }
            }

            other = tail.take();
            queue.push(head);
            head = other;
            tail = &mut head;
        }

        let mut tail = head;
        while let Some(mut v) = queue.pop() {
            while let Some(mut node) = v {
                v = node.next.take();
                node.next = tail;
                tail = Some(node);
            }
        }

        tail
    }
}
// @lc code=end

