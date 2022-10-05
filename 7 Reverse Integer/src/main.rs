fn main() {
  println!("{}", Solution::reverse(123) == 321);
  println!("{}", Solution::reverse(321) == 123);
  println!("{}", Solution::reverse(-321) == -123);
}

struct Solution {}

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let is_negative = x.is_negative();
    let mut val = x;

    if is_negative {
      val = -x;
    }

    let mut res = val
      .to_string()
      .chars()
      .rev()
      .collect::<String>()
      .parse::<i32>()
      .unwrap_or(0);

    if is_negative {
      res = -res;
    }

    res
  }
}
