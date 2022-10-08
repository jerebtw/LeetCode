fn main() {
  assert_eq!(
    Solution::swap_pairs(vec_to_listnode(vec![1, 2, 3, 4])),
    vec_to_listnode(vec![2, 1, 4, 3]),
    ""
  );
  assert_eq!(
    Solution::swap_pairs(vec_to_listnode(vec![])),
    vec_to_listnode(vec![]),
    ""
  );
  assert_eq!(
    Solution::swap_pairs(vec_to_listnode(vec![1])),
    vec_to_listnode(vec![1]),
    ""
  );
  assert_eq!(
    Solution::swap_pairs(vec_to_listnode(vec![1, 2, 3])),
    vec_to_listnode(vec![2, 1, 3]),
    ""
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

struct Solution {}

impl Solution {
  pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    vec_to_listnode(
      listnode_to_vec(head)
        .chunks(2)
        .map(|x| {
          let mut x = x.to_owned();
          x.reverse();
          x
        })
        .flatten()
        .collect::<Vec<_>>(),
    )
  }
}

fn listnode_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
  if head.is_none() {
    return vec![];
  }

  let mut return_data = vec![];
  let mut current = head.unwrap();
  loop {
    return_data.push(current.val);
    if current.next.is_some() {
      current = current.next.unwrap();
    } else {
      break;
    }
  }
  return_data.reverse();
  return_data
}

fn vec_to_listnode(res: Vec<i32>) -> Option<Box<ListNode>> {
  let mut head: Option<Box<ListNode>> = None;
  for val in res {
    head = Some(Box::new(ListNode { next: head, val }));
  }

  head
}
