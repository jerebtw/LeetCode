use std::{collections::HashSet, hash::Hash};

fn main() {
  assert_eq!(
    Solution::is_valid_sudoku(vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]),
    true
  );
  assert_eq!(
    Solution::is_valid_sudoku(vec![
      vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]),
    false
  );
  assert_eq!(
    Solution::is_valid_sudoku(vec![
      vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
      vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
      vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
      vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
    ]),
    false
  );
}

struct Solution {}

impl Solution {
  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row = HashSet::new();
    for i in 0..board.len() {
      for j in 0..board[i].len() {
        let cell = board[i][j];
        if cell != '.' && row.contains(&cell) {
          return false;
        }

        row.insert(cell);
        if j != 0 && j % 9 == 0 {
          row.clear();
        }
      }

      row.clear();
    }

    let mut column = HashSet::new();
    for i in 0..board.len() {
      for j in 0..board[i].len() {
        let cell = board[j][i];
        if cell != '.' && column.contains(&cell) {
          return false;
        }

        column.insert(cell);
        if j != 0 && j % 9 == 0 {
          column.clear();
        }
      }

      column.clear();
    }

    let flatten_board: Vec<char> = board.iter().flatten().map(|&x| x).collect();
    let i_chunks = flatten_board.chunks(27);

    let mut squares: Vec<Vec<char>> = vec![];
    let mut first = vec![];
    let mut second = vec![];
    let mut thrid = vec![];
    for i_chunk in i_chunks {
      for j_chunk in i_chunk.to_vec().chunks(9) {
        let j_vec = j_chunk.to_vec();
        first.append(&mut j_vec[0..3].to_vec());
        second.append(&mut j_vec[3..6].to_vec());
        thrid.append(&mut j_vec[6..9].to_vec());
      }

      squares.push(first);
      squares.push(second);
      squares.push(thrid);

      first = vec![];
      second = vec![];
      thrid = vec![];
    }

    for square in squares {
      if !has_unique_elements(square.iter().filter(|&&x| x != '.')) {
        return false;
      }
    }

    true
  }
}

fn has_unique_elements<T>(iter: T) -> bool
where
  T: IntoIterator,
  T::Item: Eq + Hash,
{
  let mut uniq = HashSet::new();
  iter.into_iter().all(move |x| uniq.insert(x))
}
