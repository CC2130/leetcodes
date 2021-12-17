/*
 * @lc app=leetcode.cn id=30 lang=rust
 *
 * [30] 串联所有单词的子串
 */

// @lc code=start
use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut dict: HashMap<&str, i32> = HashMap::new();
        let mut fc: HashSet<char> = HashSet::new();
        let size = words[0].len();
        let len = words.len();
        let max = size * len;

        for word in &words {
            *dict.entry(word).or_insert(0) += 1;
            fc.insert(word.chars().next().unwrap());
        }

        let buffer = s.chars().into_iter().collect::<Vec<_>>();
        let mut i = 0;

        let mut res = vec![];
        let mut dc = dict.clone();
        while i < buffer.len() - max + 1 {
            let mut j = i;
            while dc.contains_key(&s[j..j + size]) {
                let c = dc.get_mut(&s[j..j + size]).unwrap();
                if *c == 0 {
                    dc = dict.clone();
                    break
                } else {
                    *c -= 1;
                }
                j += size;
                if j == i + max {
                    dc = dict.clone();
                    res.push(i as i32);
                    break;
                }
            }
            i += 1;
            //dc = dict.clone();
        }

        res
    }
}
// @lc code=end

