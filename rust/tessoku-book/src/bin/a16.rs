// A16 Dungeon 1
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38500681

use proconio::input;
use std::cmp::min;

fn main() {
    input!{
        n: usize,
        a:[u32; n - 1],
        b:[u32; n - 2],
    }
    let mut dp = vec![10_u32.pow(8); n];
    dp[0] = 0;
    for i in 0..(n - 1) {
        dp[i + 1] = min(dp[i + 1], dp[i] + a[i]);
        if i < n - 2 {
            dp[i + 2] = min(dp[i + 2], dp[i] + b[i]);
        }
    }
    println!("{}", dp[n - 1]);
}
