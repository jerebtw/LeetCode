fn main() {
  assert_eq!(Solution::is_palindrome(121), true, "121");
  assert_eq!(Solution::is_palindrome(-121), false, "-121");
  assert_eq!(Solution::is_palindrome(10), false, "10");
  assert_eq!(Solution::is_palindrome(101), true, "101");
  assert_eq!(Solution::is_palindrome(0), true, "0");
  assert_eq!(Solution::is_palindrome(123), false, "123");
  assert_eq!(Solution::is_palindrome(1001), true, "1001");
  assert_eq!(Solution::is_palindrome(1203), false, "1203");
}

struct Solution {}

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x != 0 && (x.is_negative() || x % 10 == 0) {
      return false;
    }

    let x_str = x.to_string();
    if !x_str
      .chars()
      .into_iter()
      .eq(x_str.chars().into_iter().rev())
    {
      return false;
    }

    true
  }
}
