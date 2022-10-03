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
    nums1.append(&mut nums2);
    nums1.sort();

    let len_both = nums1.len();
    let half = len_both / 2;
    if len_both % 2 == 0 {
      return ((nums1[half - 1] as f64) + (nums1[half] as f64)) / 2.0;
    } else {
      return nums1[half] as f64;
    }
  }
}
