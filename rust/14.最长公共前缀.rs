/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut vs = vec![];
        let mut len = usize::MAX;
        for s in strs {
            let buffer = s.as_bytes().to_vec();
            if buffer.len() < len {
                len = buffer.len();
            }
            vs.push(buffer);
        }

        let mut common = vec![];
        for i in 0..len {
            let n = vs[0][i];
            for j in 1..vs.len() {
                if vs[j][i] != n {
                    return String::from_utf8(common).unwrap()
                }
            }
            common.push(n);
        }
        
        String::from_utf8(common).unwrap()
    }
}
// @lc code=end

