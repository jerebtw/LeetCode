use std::collections::HashMap;

fn main() {
  assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
  assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}

struct Solution {}

impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut stack = HashMap::new();

    for num in nums {
      if stack.contains_key(&num) {
        *stack.get_mut(&num).unwrap() += 1;
      } else {
        stack.insert(num, 1);
      }
    }

    for (key, val) in stack {
      if val == 1 {
        return key;
      }
    }
    0
  }
}
