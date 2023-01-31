// A19 Knapsack 1
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38503816

use proconio::input;

#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {
        {
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        }
    };
}

fn main() {
    input!{
        n: usize,
        w: usize,
        wv:[[usize; 2]; n],
    }
    let _dp = vec![0_usize; w + 1];
    let mut dp = vec![_dp; n + 1];
    for i in 0..n {
        for j in 0..=w {
            if j + wv[i][0] <= w {
                chmax!(dp[i + 1][j + wv[i][0]], dp[i][j] + wv[i][1]);
            }
            chmax!(dp[i + 1][j], dp[i][j]);
        }
    }
    println!("{}", dp[n][w]);
}
