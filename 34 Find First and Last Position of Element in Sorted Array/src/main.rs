fn main() {
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
    vec![3, 4]
  );
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
    vec![-1, -1]
  );
  assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}

struct Solution {}

impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if !nums.contains(&target) {
      return vec![-1, -1];
    }

    for i in 0..nums.len() {
      if nums[i] == target {
        let mut end = i;
        for j in i..nums.len() {
          if nums[j] == target {
            end = j;
          }
        }

        return vec![i as i32, end as i32];
      }
    }

    vec![-1, -1]
  }
}
