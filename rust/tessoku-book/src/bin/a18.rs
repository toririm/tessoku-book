// A18 Subset Sum
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38502849

use proconio::input;

fn main() {
    input!{
        n: usize,
        s: usize,
        a:[usize; n],
    }
    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..(s + 1) {
            if j + a[i] <= s {
                dp[i + 1][j + a[i]] += dp[i][j];
            }
            dp[i + 1][j] += dp[i][j];
        }
    }
    if dp[n][s] > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
