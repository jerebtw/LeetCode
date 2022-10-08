fn main() {
  assert_eq!(
    Solution::merge_two_lists(
      vec_to_listnode(vec![1, 2, 4]),
      vec_to_listnode(vec![1, 3, 4]),
    ),
    vec_to_listnode(vec![4, 4, 3, 2, 1, 1]),
    "{:?} {:?}",
    vec![1, 2, 4],
    vec![1, 3, 4]
  );
  assert_eq!(
    Solution::merge_two_lists(vec_to_listnode(vec![]), vec_to_listnode(vec![]),),
    vec_to_listnode(vec![]),
    "[] []",
  );
  assert_eq!(
    Solution::merge_two_lists(vec_to_listnode(vec![]), vec_to_listnode(vec![0]),),
    vec_to_listnode(vec![0]),
    "[] {:?}",
    vec![0]
  );
}

struct Solution {}

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

impl Solution {
  pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut list = listnode_to_vec(list1);
    list.append(&mut listnode_to_vec(list2));
    list.sort();
    list.reverse();

    vec_to_listnode(list)
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
