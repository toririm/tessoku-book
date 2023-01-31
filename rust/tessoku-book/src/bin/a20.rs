// A20 LCS
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38504760

use proconio::input;
use proconio::marker::Chars;

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
        s: Chars,
        t: Chars,
    }
    let sl = s.len();
    let tl = t.len();
    let mut dp = vec![vec![0; tl + 1]; sl + 1];
    for i in 0..=sl {
        for j in 0..=tl {
            if i < sl && j < tl && s[i] == t[j] {
                chmax!(dp[i + 1][j + 1], dp[i][j] + 1);
            }
            if i < sl {
                chmax!(dp[i + 1][j], dp[i][j]);
            }
            if j < tl {
                chmax!(dp[i][j + 1], dp[i][j]);
            }
        }
    }
    println!("{}", dp[sl][tl]);
}
