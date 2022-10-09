fn main() {
  assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
  assert_eq!(Solution::single_number(vec![2, 1, 2]), 1);
  assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
  assert_eq!(Solution::single_number(vec![1]), 1);
}

struct Solution {}

impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut return_num = 0;
    for num in nums {
      if return_num == 0 {
        return_num = num;
        continue;
      }

      if return_num == num {
        return_num = 0;
      }
    }
    return_num
  }
}
