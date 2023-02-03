// A25 Number of Routes
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38565676

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        h: usize,
        w: usize,
        c:[Chars; h],
    }
    let mut dp = vec![vec![0_u64; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                dp[i + 1][j + 1] = 1;
                continue;
            }
            if c[i][j] == '.' {
                dp[i + 1][j + 1] = dp[i][j + 1] + dp[i + 1][j];
            }
        }
    }
    println!("{}", dp[h][w]);
}
