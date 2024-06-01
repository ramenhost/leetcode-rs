/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 *
 * https://leetcode.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (56.64%)
 * Likes:    15156
 * Dislikes: 371
 * Total Accepted:    2M
 * Total Submissions: 3.5M
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * Given the root of a binary tree, check whether it is a mirror of itself
 * (i.e., symmetric around its center).
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [1,2,2,3,4,4,3]
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [1,2,2,null,3,null,3]
 * Output: false
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the tree is in the range [1, 1000].
 * -100 <= Node.val <= 100
 * 
 * 
 * 
 * Follow up: Could you solve it both recursively and iteratively?
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
use std::collections::VecDeque;

enum BFSQueueElement {
    Node(Option<Rc<RefCell<TreeNode>>>),
    MirrorPoint,
    EndOfRow
}

enum TreeSide {
    LeftOfMirrorPoint,
    RigthOfMirrorPoint
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut bfs_queue = VecDeque::new();
        let mut row_stack: Vec<Option<i32>> = Vec::new();
        
        // root will not be none according to constraints
        let root_node = root.as_ref().unwrap().borrow();
        bfs_queue.push_back(BFSQueueElement::Node(root_node.left.clone()));
        bfs_queue.push_back(BFSQueueElement::MirrorPoint);
        bfs_queue.push_back(BFSQueueElement::Node(root_node.right.clone()));
        bfs_queue.push_back(BFSQueueElement::EndOfRow);

        let mut current_side = TreeSide::LeftOfMirrorPoint;
        while let Some(ele) = bfs_queue.pop_front() {
            match ele {
                BFSQueueElement::Node(node) => {
                    match current_side {
                        TreeSide::LeftOfMirrorPoint => {
                            row_stack.push(node.as_ref().map(|n| n.borrow().val));
                        }
                        TreeSide::RigthOfMirrorPoint => {
                            let mirrored = row_stack.pop().unwrap();
                            if node.as_ref().map(|n| n.borrow().val) != mirrored { return false; }
                        }
                    };
                    if let Some(node) = node {
                        let node = node.as_ref().borrow();
                        bfs_queue.push_back(BFSQueueElement::Node(node.left.clone()));
                        bfs_queue.push_back(BFSQueueElement::Node(node.right.clone()));
                    }
                },
                BFSQueueElement::MirrorPoint => {
                    bfs_queue.push_back(BFSQueueElement::MirrorPoint);
                    current_side = TreeSide::RigthOfMirrorPoint;
                },
                BFSQueueElement::EndOfRow => {
                    if row_stack.len() != 0 { return false; }
                    if bfs_queue.len() <= 1 { return true; }
                    bfs_queue.push_back(BFSQueueElement::EndOfRow);
                    current_side = TreeSide::LeftOfMirrorPoint;
                }
            }
        }

        true
    }
}
// @lc code=end

