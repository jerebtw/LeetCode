use std::time::Instant;

fn main() {
    let start = Instant::now();
    for _ in 0..10_000_000 {
        Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    }
    let test = vec!["Test", "Test"];

    println!("{:?}", start.elapsed());
}

struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_val = 0;

        for i in 0..accounts.len() {
            let mut val = 0;
            for j in 0..accounts[i].len() {
                val += accounts[i][j];
            }

            if max_val < val {
                max_val = val;
            }
        }

        max_val
    }
}
