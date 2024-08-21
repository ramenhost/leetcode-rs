/*
* @lc app=leetcode id=146 lang=rust
*
* [146] LRU Cache
*
* https://leetcode.com/problems/lru-cache/description/
*
* algorithms
* Medium (42.77%)
* Likes:    20824
* Dislikes: 1047
* Total Accepted:    1.7M
* Total Submissions: 4M
* Testcase Example:  '["LRUCache","put","put","get","put","get","put","get","get","get"]\n' +
 '[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]'
*
* Design a data structure that follows the constraints of a Least Recently
* Used (LRU) cache.
*
* Implement the LRUCache class:
*
*
* LRUCache(int capacity) Initialize the LRU cache with positive size
* capacity.
* int get(int key) Return the value of the key if the key exists, otherwise
* return -1.
* void put(int key, int value) Update the value of the key if the key exists.
* Otherwise, add the key-value pair to the cache. If the number of keys
* exceeds the capacity from this operation, evict the least recently used
* key.
*
*
* The functions get and put must each run in O(1) average time complexity.
*
*
* Example 1:
*
*
* Input
* ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
* [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
* Output
* [null, null, null, 1, null, -1, null, -1, 3, 4]
*
* Explanation
* LRUCache lRUCache = new LRUCache(2);
* lRUCache.put(1, 1); // cache is {1=1}
* lRUCache.put(2, 2); // cache is {1=1, 2=2}
* lRUCache.get(1);    // return 1
* lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
* lRUCache.get(2);    // returns -1 (not found)
* lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
* lRUCache.get(1);    // return -1 (not found)
* lRUCache.get(3);    // return 3
* lRUCache.get(4);    // return 4
*
*
*
* Constraints:
*
*
* 1 <= capacity <= 3000
* 0 <= key <= 10^4
* 0 <= value <= 10^5
* At most 2 * 10^5 calls will be made to get and put.
*
*
*/

// @lc code=start

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: Copy> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

struct DoublyLinkedList<T: Copy> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: T) -> Rc<RefCell<Node<T>>> {
        let node = Rc::new(RefCell::new(Node::new(value)));
        if let Some(tail) = self.tail.take() {
            tail.as_ref().borrow_mut().next = Some(Rc::clone(&node));
            node.as_ref().borrow_mut().prev = Some(tail);
        }
        self.tail = Some(Rc::clone(&node));
        if self.head.is_none() {
            self.head = Some(Rc::clone(&node));
        }
        node
    }

    fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take();
        self.head = head.as_ref()?.as_ref().borrow_mut().next.take();
        if let Some(head) = self.head.as_ref() {
            head.as_ref().borrow_mut().prev = None;
        }
        head.map(|h| h.as_ref().borrow().value)
    }

    fn move_back(&mut self, node: &Rc<RefCell<Node<T>>>) {
        let mut node_mut = node.as_ref().borrow_mut();
        let mut tail = None;
        if node_mut.next.is_none() {
            return;
        }
        if let Some(prev) = node_mut.prev.take() {
            let mut prev_mut = prev.as_ref().borrow_mut();
            self.tail.as_ref().unwrap().as_ref().borrow_mut().next = prev_mut.next.take();
            prev_mut.next = node_mut.next.take();
            let mut next_mut = prev_mut.next.as_ref().unwrap().as_ref().borrow_mut();
            tail = next_mut.prev.take();
            next_mut.prev = Some(Rc::clone(&prev));
        } else if let Some(next) = node_mut.next.take() {
            let mut next_mut = next.as_ref().borrow_mut();
            let cur_node = next_mut.prev.take();
            next_mut.prev = None;
            drop(next_mut);
            self.tail.as_ref().unwrap().as_ref().borrow_mut().next = cur_node;
            tail = self.head.take();
            self.head = Some(next);
        }
        node_mut.next = None;
        node_mut.prev = self.tail.take();
        drop(node_mut);
        self.tail = tail;
    }
}

struct LRUCache {
    capacity: i32,
    h_map: HashMap<i32, (i32, Rc<RefCell<Node<i32>>>)>,
    usage_list: DoublyLinkedList<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            h_map: HashMap::new(),
            usage_list: DoublyLinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some((value, node)) = self.h_map.get(&key) {
            self.usage_list.move_back(node);
            return *value;
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some((entry, node)) = self.h_map.get_mut(&key) {
            *entry = value;
            self.usage_list.move_back(node);
            return;
        }
        if self.h_map.len() as i32 >= self.capacity {
            let lru_node = self.usage_list.pop_front().unwrap();
            self.h_map.remove(&lru_node);
        }
        let usage_node = self.usage_list.push_back(key);
        self.h_map.insert(key, (value, usage_node));
    }
}

// @lc code=end
