fn main() {
  assert_eq!(
    Solution::longest_common_prefix(vec![
      "flower".to_string(),
      "flow".to_string(),
      "flight".to_string()
    ]),
    "fl",
    "[\"flower\",\"flow\",\"flight\"]"
  );
  assert_eq!(
    Solution::longest_common_prefix(vec![
      "flower".to_string(),
      "flower".to_string(),
      "flower".to_string()
    ]),
    "flower",
    "[\"flower\",\"flower\",\"flower\"]"
  );
  assert_eq!(
    Solution::longest_common_prefix(vec!["a".to_string()]),
    "a",
    "[\"a\"]"
  );
  assert_eq!(
    Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
    "a",
    "[\"a\",\"ab\"]"
  );
}

struct Solution {}

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
      return "".to_string();
    } else if strs.len() == 1 {
      return strs.get(0).unwrap().to_string();
    }
    let mut prefix = "";
    let mut strs = strs;

    strs.sort_by(|a, b| a.len().cmp(&b.len()));

    let first_str = strs.get(0).unwrap();
    'i_loop: for i in 0..first_str.len() {
      let current_str = &first_str[0..(i + 1)];
      for j_str in &strs {
        if &j_str[0..(i + 1)] != current_str {
          break 'i_loop;
        }
      }

      prefix = &current_str;
    }

    prefix.to_string()
  }
}
