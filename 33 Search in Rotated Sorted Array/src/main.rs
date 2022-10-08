fn main() {
  assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
  assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
  assert_eq!(Solution::search(vec![1], 0), -1);
}

struct Solution {}

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if !nums.contains(&target) {
      return -1;
    }

    for i in 0..nums.len() {
      if nums[i] == target {
        return i as i32;
      }
    }

    -1
  }
}
