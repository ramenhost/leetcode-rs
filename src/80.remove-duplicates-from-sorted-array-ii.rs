/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let (mut i, mut j) = (2, 2);
        if nums_len < 3 {
            return nums_len as i32;
        }
        while j < nums_len {
            if nums[j] == nums[i-1] && nums[j] == nums[i-2] {
                j += 1;
            } else {
                nums[i] = nums[j];
                i += 1;
                j += 1;
            }
        }
        i as i32
    }
}
// @lc code=end

