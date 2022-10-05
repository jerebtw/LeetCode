fn main() {
  assert_eq!(
    Solution::length_of_longest_substring("abcabcbb".to_string()),
    3,
    "abcabcbb"
  );
  assert_eq!(
    Solution::length_of_longest_substring("bbbbb".to_string()),
    1,
    "bbbbb"
  );
  assert_eq!(
    Solution::length_of_longest_substring("pwwkew".to_string()),
    3,
    "pwwkew"
  );
  assert_eq!(
    Solution::length_of_longest_substring("a".to_string()),
    1,
    "a"
  );
  assert_eq!(
    Solution::length_of_longest_substring(" ".to_string()),
    1,
    " "
  );
}

struct Solution {}

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<String> = s.chars().map(|x| x.to_string()).collect();
    let chars_len = chars.len();
    let mut max_try = 0;

    for i in 0..chars_len {
      let mut found_chars = vec![];
      let mut found_dop = false;
      for j in i..chars_len {
        let s_char = &chars[j];
        if found_chars.contains(&s_char) {
          let diff = j - i;
          found_dop = true;
          if max_try < diff {
            max_try = diff;
          }
          break;
        }

        found_chars.push(s_char);
      }

      if !found_dop && max_try < found_chars.len() {
        max_try = found_chars.len();
      }
    }

    max_try as i32
  }
}
