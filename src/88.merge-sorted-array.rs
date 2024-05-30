/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        nums1.drain(m..);
        let mut merged: Vec<i32> = Vec::with_capacity(m + n);
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        while i < m {
            merged.push(nums1[i]);
            i += 1;
        }
        while j < n {
            merged.push(nums2[j]);
            j += 1;
        }
        nums1.clear();
        nums1.extend_from_slice(&merged[..]);

    }
}
// @lc code=end

