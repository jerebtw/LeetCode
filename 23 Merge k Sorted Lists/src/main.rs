use std::time::Instant;

fn main() {
  let start = Instant::now();
  assert_eq!(
    Solution::merge_k_lists(vec![
      vec_to_listnode(vec![1, 4, 5]),
      vec_to_listnode(vec![1, 3, 4]),
      vec_to_listnode(vec![2, 6]),
    ]),
    vec_to_listnode(vec![6, 5, 4, 4, 3, 2, 1, 1])
  );
  assert_eq!(Solution::merge_k_lists(vec![]), vec_to_listnode(vec![]));
  println!("{:?}", start.elapsed());
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
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let lists_flatten = lists.iter().flatten().collect::<Vec<_>>().to_owned();
    let mut list_data: Vec<i32> = vec![];
    for item in lists_flatten {
      list_data.append(&mut listnode_to_vec(Some(item.to_owned())));
    }
    list_data.sort();
    list_data.reverse();

    vec_to_listnode(list_data)
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
