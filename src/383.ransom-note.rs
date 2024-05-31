/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 *
 * https://leetcode.com/problems/ransom-note/description/
 *
 * algorithms
 * Easy (61.55%)
 * Likes:    4925
 * Dislikes: 497
 * Total Accepted:    1.2M
 * Total Submissions: 1.9M
 * Testcase Example:  '"a"\n"b"'
 *
 * Given two strings ransomNote and magazine, return true if ransomNote can be
 * constructed by using the letters from magazine and false otherwise.
 * 
 * Each letter in magazine can only be used once in ransomNote.
 * 
 * 
 * Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= ransomNote.length, magazine.length <= 10^5
 * ransomNote and magazine consist of lowercase English letters.
 * 
 * 
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut available = HashMap::new();
        magazine.chars().for_each(|c| {
            available.entry(c).and_modify(|count| *count += 1).or_insert(1);
        });
        for c in ransom_note.chars() {
            let rem = available.entry(c).and_modify(|count| *count -= 1).or_insert(-1);
            if *rem < 0 { return false; }
        }
        true
    }
}
// @lc code=end

