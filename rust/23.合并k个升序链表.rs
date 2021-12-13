/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
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
use std::cmp::Ordering;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl ListNode {
    fn to_vec(mut lns: Option<Box<Self>>) -> Vec<Self> {
        let mut v = vec![];
        while let Some(mut ln) = lns {
            lns = ln.next.take();
            v.push(*ln);
        }

        v
    }

    fn from_vec(mut v: Vec<Self>) -> Option<Box<Self>> {
        let mut head = None;
        while let Some(mut tail) = v.pop() {
            tail.next = head;
            head = Some(Box::new(tail));
        }

        head
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut v = vec![];
        for l in lists {
            if let Some(l) = l {
                v.append(&mut ListNode::to_vec(Some(l)));
            }
        }

        v.sort();
        ListNode::from_vec(v)
    }
}
// @lc code=end

