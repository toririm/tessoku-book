// A21 Block Game
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38510165

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
        mut pa:[(usize, u32); n],
    }
    pa.insert(0, (0, 0)); // 1-based
    let mut dp = vec![vec![0_u32; n + 1]; n + 1];
    for len in (1..=(n - 1)).rev() { // len = r - l
        for l in 1..=(n - len) {
            let r = l + len;
            let mut rpoint = 0;
            if l <= pa[r].0 && pa[r].0 <= r {
                rpoint = pa[r].1;
            }
            let mut lpoint = 0;
            if l <= pa[l].0 && pa[l].0 <= r {
                lpoint = pa[l].1;
            }
            chmax!(dp[l][r - 1], dp[l][r] + rpoint);
            chmax!(dp[l + 1][r], dp[l][r] + lpoint);
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        chmax!(ans, dp[i][i]);
    }
    println!("{}", ans);
}
