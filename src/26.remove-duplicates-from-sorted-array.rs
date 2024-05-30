/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut added = HashSet::new();
        let mut filtered = vec![];
        for n in nums.iter() {
            if !added.contains(n) {
                filtered.push(*n);
                added.insert(*n);
            }
        }
        nums.clear();
        nums.extend_from_slice(&filtered[..]);
        nums.len() as i32
    }
}
// @lc code=end

