fn main() {
  println!(
    "{}",
    Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
  );
  println!(
    "{}",
    Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
  );
}

struct Solution {}

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    let mut both = vec![];
    both.append(&mut nums1);
    both.append(&mut nums2);
    both.sort();

    let len_both = both.len();
    if len_both % 2 == 0 {
      return ((both[len_both / 2 - 1] as f64) + (both[len_both / 2] as f64)) / 2.0;
    } else {
      return both[len_both / 2] as f64;
    }
  }
}
