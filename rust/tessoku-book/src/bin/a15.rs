// A15 Compression
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38500094

use proconio::input;
use std::collections::HashSet;

fn main() {
    input!{
        n: usize,
        a:[u32; n],
    }
    let c: HashSet<u32> = a.to_vec().into_iter().collect();
    let mut d: Vec<u32> = c.into_iter().collect();
    d.sort();
    for i in 0..n {
        print!("{}{}", d.binary_search(&(a[i])).unwrap() + 1, if i == n - 1 { "\n" } else { " " } );
    }
}
