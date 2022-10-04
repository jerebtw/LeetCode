use std::num::IntErrorKind;

fn main() {
  println!("{}", Solution::my_atoi("42".to_string()));
  println!("{}", Solution::my_atoi("0042".to_string()));
  println!("{}", Solution::my_atoi("   -42".to_string()));
  println!("{}", Solution::my_atoi("4193 with words".to_string()));
  println!("{}", Solution::my_atoi("-91283472332".to_string()));
  println!("{}", Solution::my_atoi("3.14159".to_string()));
  println!("{}", Solution::my_atoi("".to_string()));
  println!("{}", Solution::my_atoi("  -0012a42".to_string()));
  println!("{}", Solution::my_atoi("TEst12".to_string()));
  println!("{}", Solution::my_atoi("-5-".to_string()));
  println!("{}", Solution::my_atoi("123-".to_string()));
}

struct Solution {}

impl Solution {
  pub fn my_atoi(s: String) -> i32 {
    let s_trim = s.trim();
    if s_trim == "" {
      return 0;
    }

    let mut s_ret: String = String::new();
    let mut found_num = false;
    for char in s_trim.chars() {
      let char = char.to_string();
      if char.parse::<i32>().is_ok() {
        found_num = true;
        s_ret += &char;
        continue;
      }

      if char == "-" || char == "+" {
        if !found_num {
          s_ret += &char;
          continue;
        }
      }

      break;
    }

    let res = s_ret.parse::<i32>();

    if let Err(e) = res.clone() {
      match e.kind() {
        IntErrorKind::NegOverflow => return i32::MIN,
        IntErrorKind::PosOverflow => return i32::MAX,
        IntErrorKind::Empty => return 0,
        IntErrorKind::InvalidDigit => return 0,
        IntErrorKind::Zero => return 0,
        _ => return 0,
      };
    }
    if let Ok(res) = res {
      return res;
    }

    0
  }
}
