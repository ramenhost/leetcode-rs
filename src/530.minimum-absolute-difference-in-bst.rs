/*
 * @lc app=leetcode id=530 lang=rust
 *
 * [530] Minimum Absolute Difference in BST
 *
 * https://leetcode.com/problems/minimum-absolute-difference-in-bst/description/
 *
 * algorithms
 * Easy (58.36%)
 * Likes:    4342
 * Dislikes: 221
 * Total Accepted:    380.7K
 * Total Submissions: 652.4K
 * Testcase Example:  '[4,2,6,1,3]'
 *
 * Given the root of a Binary Search Tree (BST), return the minimum absolute
 * difference between the values of any two different nodes in the tree.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [4,2,6,1,3]
 * Output: 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [1,0,48,null,null,12,49]
 * Output: 1
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the tree is in the range [2, 10^4].
 * 0 <= Node.val <= 10^5
 * 
 * 
 * 
 * Note: This question is the same as 783:
 * https://leetcode.com/problems/minimum-distance-between-bst-nodes/
 * 
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{min, max};

impl Solution {
    fn get_mindiff_smallest_and_largest(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        let root_brw = root.as_ref().unwrap().borrow();
        let mut min_diff = i32::MAX;
        let mut largest = root_brw.val;
        let mut smallest = root_brw.val;

        if let Some(left) = root_brw.left.as_ref() {
            let (left_mindiff, left_smallest, left_largest) = Self::get_mindiff_smallest_and_largest(Some(left.clone()));
            min_diff = min(min_diff, left_mindiff);
            min_diff = min(min_diff, (root_brw.val - left_largest).abs());
            smallest = min(smallest, left_smallest);
        }
        if let Some(right) = root_brw.right.as_ref() {
            let (right_mindiff, right_smallest, right_largest) = Self::get_mindiff_smallest_and_largest(Some(right.clone()));
            min_diff = min(min_diff, right_mindiff);
            min_diff = min(min_diff, (root_brw.val - right_smallest).abs());
            largest = max(largest, right_largest);
        }
        (min_diff, smallest, largest)
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_mindiff_smallest_and_largest(root).0
    }
}
// @lc code=end

