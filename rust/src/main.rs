use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let s = String::from("aacabdkacaa");
    let s = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    longest_palindrome(s);
    test();
}

fn test() {
    let a = Rc::new(45);
    let b = a.clone();
    if a == b {
        println!("a == b");
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


pub fn longest_palindrome(s: String) -> String {
    let _buffer = s.as_bytes();
    if _buffer.len() == 1 {
        return s
    }

    let mut buffer = Vec::new();
    let mut store: HashMap<u8, Vec<usize>> = HashMap::new();
    let mut pstore: HashMap<u8, Rc<u8>> = HashMap::new();
    for i in 0.._buffer.len() {
        if let Some(arr) = store.get_mut(&_buffer[i]) {
            arr.push(i);
            buffer.push(pstore[&_buffer[i]].clone());
        } else {
            pstore.insert(_buffer[i], Rc::new(_buffer[i]));
            store.insert(_buffer[i], vec![i]);
            buffer.push(pstore[&_buffer[i]].clone());
        }
    }

    let mut singles: Vec<usize> = Vec::new();
    for e in store.iter().filter(|x| x.1.len() == 1) {
       singles.push(e.1[0]);
    }
    singles.sort();
    println!("{:?}", singles);

    let mut res = String::from_utf8(vec![_buffer[0]]).unwrap();
    let mut head = 0;
    let mut tail;
    let mut c;
    let mut indexs;
    let mut i;
    let mut len;
    let mut max = 1;
    let mut next;
    // 从左开始
    while head < buffer.len() {
        // 得到当前位置的元素
        c = *buffer[head];
        indexs = store.get(&c).unwrap();
        // 长度为 1 的略过
        if indexs.len() == 1 {
            head += 1;
            continue;
        }
        i = indexs.len() - 1;

        println!("head: {}", head);
        while head < indexs[i] {
            tail = indexs[i];
            len = tail - head + 1;
            // 后面都不用试了，下一个元素 i += 1
            if len <= max {
                break
            }
            next = false;
            // 单个元素位置检查
            for i in 0..singles.len() {
                let i = singles[i];
                println!("{}", i);
                if head < i && i < tail {
                    if i != (head + tail) / 2 {
                        next = true;
                        break;
                    }
                }
            }

            if next {
                i -= 1;
                continue;
            }
            //
            let mut l = head;
            let mut r = tail;
            // 这里开始匹配
            loop {
                l += 1;
                r -= 1;
                if l > r {
                    if max < len {
                        max = len;
                        println!("max: {}", max);
                        res = String::from_utf8(_buffer[head..tail + 1].to_vec()).unwrap()
                    }
                    break
                }
                if buffer[l] != buffer[r] {
                    break
                }
            }
            // 匹配失败，tail 左移再来
            i -= 1;
        }

        //println!("head: {}", head);
        head += 1;
    }

    res
}