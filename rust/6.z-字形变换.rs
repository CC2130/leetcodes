/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s
        }
        let buffer = s.as_bytes();
        if buffer.len() <= num_rows {
            return s
        }

        // init lines: <Vec<Vec<u8>>>, circle
        let mut lines = vec![];
        let mut circle = vec![];
        for i in 0..num_rows {
            lines.push(vec![]);
            circle.push(i);
        }

        for i in 1..num_rows - 1 {
            circle.insert(num_rows, i)
        }

        // work here
        let ct = buffer.len() / circle.len();
        let mut i = 0;
        for _ in 0..ct {
            for j in 0..circle.len() {
                lines[circle[j]].push(buffer[i]);
                i += 1;
            }
        }

        for j in 0..buffer.len() - ct * circle.len() {
            lines[circle[j]].push(buffer[i]);
            i += 1;
        }

        // combine
        let mut v = vec![];
        for i in 0..num_rows {
            v.append(&mut lines[i]);
        }
        let res = String::from_utf8(v).unwrap();
        res
    }

    // use char
    //pub fn convert(s: String, num_rows: i32) -> String {
    //    if num_rows == 1 {
    //        return s
    //    }
    //    let mut chars: Vec<char> = s.chars().collect();
    //    if chars.len() < num_rows as usize {
    //        return s
    //    }

    //    // init lines & circle
    //    let mut lines = Vec::new();
    //    let mut circle = Vec::new();
    //    for i in 0..num_rows {
    //        lines.push(String::new());
    //        circle.push(i as usize);
    //    }

    //    for i in 1..num_rows - 1 {
    //        circle.insert(num_rows as usize, i as usize);
    //    }

    //    let circle_times = chars.len() / circle.len() as usize;
    //    for i in 0..circle_times {
    //        for j in 0..circle.len() {
    //            lines[circle[j]].push(chars.remove(0));
    //        }
    //    }

    //    for i in 0..chars.len() {
    //        lines[circle[i]].push(chars.remove(0))
    //    }

    //    let mut res = String::new();
    //    for i in 0..num_rows as usize {
    //        res.push_str(&lines[i]);
    //    }

    //    res
    //}
}
// @lc code=end

