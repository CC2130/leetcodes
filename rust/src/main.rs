fn main() {
    println!("Hello, world!");
    test();
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn from_nums(nums: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        for i in 0..nums.len() {
            *tail = Some(Box::new(ListNode::new(nums[i])));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }

}

fn test() {
    let arr = vec![1, 2];
    let mut res = ListNode::from_nums(&arr);
    let mut p = &mut res;

    let mut i = 0;
    let k = 1;
    while i < k{
        if let Some(n) = p {
            p = &mut n.as_mut().next;
            println!("{:?}", p);
            i += 1;
        } else {
            p = &mut res;
        }
    }
    let mut h = p.as_mut().unwrap().next.take();
    println!("{:?}", h);

    p = &mut h;
    while let Some(n) = p {
        p = &mut n.as_mut().next;
    }
    *p = res;

    println!("{:?}", h);
    //println!("{:?}", res);

}