fn main() {
  let first = &mut vec![1, 2, 3, 4, 5, 6, 7];
  Solution::rotate(first, 3);
  assert_eq!(first, &mut vec![5, 6, 7, 1, 2, 3, 4]);
  let first = &mut vec![-1, -100, 3, 99];
  Solution::rotate(first, 2);
  assert_eq!(first, &mut vec![3, 99, -1, -100]);
}

struct Solution {}

impl Solution {
  pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    for _ in 0..k {
      let pop_data = nums.pop();
      if pop_data.is_some() {
        nums.insert(0, pop_data.unwrap());
      }
    }
  }
}
