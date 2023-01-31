// A16 Dungeon 2
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38501689

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
    let mut p = Vec::new();
    p.push(n);
    let mut i = n - 1;
    while i > 0 {
        if dp[i] == dp[i - 1] + a[i - 1] {
            p.push(i);
            i = i - 1;
        }
        if i > 1 && dp[i] == dp[i - 2] + b[i - 2] {
            p.push(i - 1);
            i = i - 2;
        }
    }
    p.reverse();
    let k = p.len();
    println!("{}", k);
    for i in 0..k {
        print!("{}{}", p[i], if i == k - 1 { "\n" } else { " " });
    }
}
