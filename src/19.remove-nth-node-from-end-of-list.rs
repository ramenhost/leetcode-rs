/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 *
 * https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (45.51%)
 * Likes:    18743
 * Dislikes: 792
 * Total Accepted:    2.8M
 * Total Submissions: 6.1M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * Given the head of a linked list, remove the n^th node from the end of the
 * list and return its head.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [1,2,3,4,5], n = 2
 * Output: [1,2,3,5]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [1], n = 1
 * Output: []
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: head = [1,2], n = 1
 * Output: [1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the list is sz.
 * 1 <= sz <= 30
 * 0 <= Node.val <= 100
 * 1 <= n <= sz
 * 
 * 
 * 
 * Follow up: Could you do this in one pass?
 * 
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
// @lc code=start
impl Solution {
    pub fn len(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut len = 0;
        while let Some(node) = head.as_ref() {
            len += 1;
            head = &node.next;
        }
        len
    }
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let nodes = Self::len(&head);
        let mut node = &mut head;
        if nodes == n {
            return head.unwrap().next;
        }
        for _ in 1..(nodes-n) {
            node = &mut node.as_mut().unwrap().next;
        }
        let mut node = node.as_mut().unwrap();
        node.next = node.next.take().unwrap().next;
        head
    }
}
// @lc code=end

