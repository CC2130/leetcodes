/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 实现 strStr()
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let buffer1 = haystack.as_bytes();
        let buffer2 = needle.as_bytes();
        let len1 = buffer1.len();
        let len2 = buffer2.len();

        if len1 < len2 {
            return -1
        }

        let end = len1 - len2;

        for i in 0..end + 1 {
            if buffer1[i..i+len2] == buffer2[0..len2] {
                return i as i32
            }
        }

        return -1
    }
}
// @lc code=end

