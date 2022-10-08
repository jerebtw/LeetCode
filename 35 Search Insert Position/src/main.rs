fn main() {
  assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
  assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
  assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
  assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
  assert_eq!(Solution::search_insert(vec![2, 3, 4, 8, 10], 0), 0);
  assert_eq!(Solution::search_insert(vec![2, 3, 4, 7, 8, 9], 11), 6);
}

struct Solution {}

impl Solution {
  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut not_found = -1;
    for (i, item) in nums.iter().enumerate() {
      if item == &target {
        return i as i32;
      }
      if item == &(target - 1) {
        not_found = i as i32 + 1;
      }
      if item == &(target + 1) {
        not_found = i as i32;
      }
    }

    if not_found == -1 {
      if nums.get(0) > Some(&target) {
        return 0;
      }
      if nums.last() < Some(&target) {
        return nums.len() as i32;
      }
    }

    not_found
  }
}
