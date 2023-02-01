// A22 Sugoroku
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38517766

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
        a:[usize; n - 1], // 0-based
        b:[usize; n - 1],
    }
    let mut dp = vec![-10_i32.pow(9); n + 1];
    dp[1] = 0; // 1-based
    for i in 0..(n - 1) {
        chmax!(dp[a[i]], dp[i + 1] + 100);
        chmax!(dp[b[i]], dp[i + 1] + 150);
    }
    println!("{}", dp[n]);
}