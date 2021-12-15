/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] 最后一个单词的长度
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let space = String::from(" ").as_bytes()[0];
        let mut c = 0;
        let buffer = s.as_bytes();
        let mut index = buffer.len() - 1;

        while buffer[index] == space {
            index -= 1;
        }

        loop {
            if buffer[index] != space {
                c += 1;
                if index == 0 {
                    break
                }
                index -= 1;
            } else {
                break
            }
        }

        c
    }
}
// @lc code=end

