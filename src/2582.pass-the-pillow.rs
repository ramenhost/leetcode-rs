/*
 * @lc app=leetcode id=2582 lang=rust
 *
 * [2582] Pass the Pillow
 *
 * https://leetcode.com/problems/pass-the-pillow/description/
 *
 * algorithms
 * Easy (46.06%)
 * Likes:    708
 * Dislikes: 33
 * Total Accepted:    104.8K
 * Total Submissions: 191.7K
 * Testcase Example:  '4\n5'
 *
 * There are n people standing in a line labeled from 1 to n. The first person
 * in the line is holding a pillow initially. Every second, the person holding
 * the pillow passes it to the next person standing in the line. Once the
 * pillow reaches the end of the line, the direction changes, and people
 * continue passing the pillow in the opposite direction.
 * 
 * 
 * For example, once the pillow reaches the n^th person they pass it to the n -
 * 1^th person, then to the n - 2^th person and so on.
 * 
 * 
 * Given the two positive integers n and time, return the index of the person
 * holding the pillow after time seconds.
 * 
 * Example 1:
 * 
 * 
 * Input: n = 4, time = 5
 * Output: 2
 * Explanation: People pass the pillow in the following way: 1 -> 2 -> 3 -> 4
 * -> 3 -> 2.
 * After five seconds, the 2^nd person is holding the pillow.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: n = 3, time = 2
 * Output: 3
 * Explanation: People pass the pillow in the following way: 1 -> 2 -> 3.
 * After two seconds, the 3^r^d person is holding the pillow.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 2 <= n <= 1000
 * 1 <= time <= 1000
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn pass_the_pillow(mut n: i32, mut time: i32) -> i32 {
        n -= 1;
        time %= n * 2;
        if time > n {
            n - (time - n) + 1
        } else {
            time + 1
        }
    }
}
// @lc code=end

