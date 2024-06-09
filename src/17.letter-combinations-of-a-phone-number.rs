/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 *
 * https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (60.61%)
 * Likes:    18377
 * Dislikes: 991
 * Total Accepted:    2.1M
 * Total Submissions: 3.4M
 * Testcase Example:  '"23"'
 *
 * Given a string containing digits from 2-9 inclusive, return all possible
 * letter combinations that the number could represent. Return the answer in
 * any order.
 * 
 * A mapping of digits to letters (just like on the telephone buttons) is given
 * below. Note that 1 does not map to any letters.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: digits = ""
 * Output: []
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: digits = "2"
 * Output: ["a","b","c"]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= digits.length <= 4
 * digits[i] is a digit in the range ['2', '9'].
 * 
 * 
 */

use std::collections::HashMap;

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let char_map = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut combinations: Vec<String> = vec![];
        for d in digits.chars() {
            let d_chars = char_map[d.to_digit(10).unwrap() as usize];
            if combinations.is_empty() {
                combinations.extend(d_chars.chars().map(|c| c.to_string()));
                continue;
            }
            let mut new_combinations: Vec<String> = vec![];
            for existing in combinations {
                for c in d_chars.chars() {
                    let mut new = existing.clone();
                    new.push(c);
                    new_combinations.push(new);
                }
            }
            combinations = new_combinations;
        }
        combinations
    }
}
// @lc code=end

