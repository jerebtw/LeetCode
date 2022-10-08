use std::time::Instant;

fn main() {
  let start = Instant::now();
  assert_eq!(Solution::is_valid("()".to_string()), true, "()");
  assert_eq!(Solution::is_valid("()[]{}".to_string()), true, "()[]{{}}");
  assert_eq!(Solution::is_valid("(]".to_string()), false, "(]");
  assert_eq!(
    Solution::is_valid("{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}".to_string()),
    true, 
    "{}",
    "{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}{}{}{}[[][][][]((()))[]]()[]{}"
  );
  println!("{:?}", start.elapsed());
}

struct Solution {}

impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut parentheses = vec![];

    for parenthese in s.chars().map(|x| x.to_string()).collect::<Vec<_>>() {
      if parenthese == "(" || parenthese == "{" || parenthese == "[" {
        parentheses.push(parenthese);
      } else {
        let last = parentheses.pop();
        if last.is_none() {
          return false;
        }

        let last = last.unwrap();
        if (last == "(" && parenthese != ")")
          || (last == "{" && parenthese != "}")
          || (last == "[" && parenthese != "]")
        {
          return false;
        }
      }
    }

    if parentheses.len() != 0 {
      return false;
    }

    true
  }
}
