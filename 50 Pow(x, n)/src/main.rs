fn main() {
  assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
  assert_eq!(Solution::my_pow(2.1, 3), 9.261);
  assert_eq!(Solution::my_pow(2.0, -2), 0.25);
}

struct Solution {}

impl Solution {
  pub fn my_pow(x: f64, n: i32) -> f64 {
    (x.powi(n) * 100000.0).round() / 100000.0
  }
}
