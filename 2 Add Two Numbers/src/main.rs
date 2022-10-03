fn main() {
  // println!("{:?}", to_list(123));
  println!(
    "{:?}",
    Solution::add_two_numbers(to_list(vec![342]), to_list(vec![465]))
  );
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub struct Solution {}

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut res = vec![];
    while l1.is_some() || l2.is_some() || carry != 0 {
      let x = l1.clone().unwrap_or(Box::new(ListNode::new(0)));
      let y = l2.clone().unwrap_or(Box::new(ListNode::new(0)));
      let sum = carry + x.val + y.val;
      carry = sum / 10;

      res.push(sum % 10);

      if l1.is_some() {
        l1 = l1.unwrap().next;
      }
      if l2.is_some() {
        l2 = l2.unwrap().next;
      }
    }

    res.reverse();
    to_list(res)
  }
}

fn to_list(res: Vec<i32>) -> Option<Box<ListNode>> {
  let mut head: Option<Box<ListNode>> = None;
  for val in res {
    head = Some(Box::new(ListNode { next: head, val }));
  }

  head
}
