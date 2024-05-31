/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (43.00%)
 * Likes:    30732
 * Dislikes: 6105
 * Total Accepted:    4.6M
 * Total Submissions: 10.7M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 * 
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
impl IntoIterator for Box<ListNode> {
    type Item = Box<ListNode>;
    type IntoIter = ListNodeIterator;

    fn into_iter(self) -> ListNodeIterator {
        ListNodeIterator {
            curr: Some(self)
        }
    }
}

pub struct ListNodeIterator {
    pub curr: Option<Box<ListNode>>
}

impl Iterator for ListNodeIterator {
    type Item = Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut to_ret = self.curr.take();
        self.curr = to_ret.as_mut()?.next.take();
        to_ret
    }
}

fn add_node_at_tail(tail: &mut ListNode, val: i32) -> Option<&mut ListNode> {
    let new_node = Box::new(ListNode::new(val));
    tail.next = Some(new_node);
    Some((&mut tail.next).as_mut().unwrap().as_mut())
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let list1 = match l1 {
            Some(val) => val,
            None => { return l2 }
        };
        let list2 = match l2 {
            Some(val) => val,
            None => { return Some(list1) }
        };

        let mut sum_list: Option<Box<ListNode>> = None;
        let mut tail: Option<&mut ListNode> = None;
        let mut carry = 0;

        let mut list1 = list1.into_iter();
        let mut list2 = list2.into_iter();
        
        // Assume list1 is longest
        for n1 in &mut list1 {
            let mut n2_val = 0;
            let mut n2_over = true;
            if let Some(n2) = list2.next() {
                n2_val = n2.val;
                n2_over = false;   
            }
            let sum = n1.val + n2_val + carry;
            carry = sum / 10;
            if tail == None {
                sum_list = Some(Box::new(ListNode::new(sum % 10)));
                tail = Some(sum_list.as_mut().unwrap());
            } else {
                tail = add_node_at_tail(tail.unwrap(), sum % 10);
            }

            if n2_over && carry <= 0 {
                tail.as_mut().unwrap().next = list1.curr;
                break;
            }
        }

        for n2 in &mut list2 {
            let sum = n2.val + carry;
            carry = sum / 10;
            tail = add_node_at_tail(tail.unwrap(), sum % 10);

            if carry <= 0 {
                tail.as_mut().unwrap().next = list2.curr;
                break;
            }
        }
                
        if carry > 0 {
            add_node_at_tail(tail.unwrap(), carry);
        }

        sum_list
    }
}
// @lc code=end

