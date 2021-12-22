/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut i = 0;
        let mut tail = &mut head;
        while let Some(node) = tail {
            tail = &mut node.next;
            i += 1;
        }
        if i == n { return head.unwrap().next.take() }

        i = i - n;
        tail = &mut head;
        while let Some(node) = tail {
            if i == 1 {
                node.next = node.next.take().unwrap().next.take();
                break
            }
            tail = &mut node.next;
            i -= 1;
        }

        head
    }
}
// @lc code=end

