fn main() {
    println!("Hello, world!");
    test();
}



use demo::ListNode;
fn test() {

    let mut t = ListNode::from_vec(vec![1,2,3,4, 5]);
    t = sp(t);
    println!("{:?}", t);
    //println!("{:?}", res);
}

fn sp(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut tail = &mut head;
    let mut other;
    let mut queue = vec![];
    'all: loop {
        for _ in 0..2 {
            if let Some(node) = tail {
                tail = &mut node.next;
            } else {
                break 'all;
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

    println!("{:?}", queue);
    tail
}