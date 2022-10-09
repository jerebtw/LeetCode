fn main() {
  let mut data = vec![2, 0, 2, 1, 1, 0];
  Solution::sort_colors(&mut data);
  assert_eq!(data, vec![0, 0, 1, 1, 2, 2]);
}

struct Solution {}

impl Solution {
  pub fn sort_colors(nums: &mut Vec<i32>) {
    nums.sort();
  }
}
