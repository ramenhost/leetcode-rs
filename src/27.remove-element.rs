/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let filtered = nums.iter().filter(|&n| *n != val).cloned().collect::<Vec<i32>>();
        nums.clear();
        nums.extend_from_slice(&filtered[..]);
        nums.len() as i32
    }
}
// @lc code=end

