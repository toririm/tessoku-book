// A23 All Free
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38519407

use proconio::input;

#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr ) => {
        {
            if $a > $b {
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
        m: usize,
        a:[[usize; n]; m]
    }
    let bits = 1 << n;
    let mut dp = vec![vec![1000; bits]; m + 1];
    dp[0][0] = 0;
    for bit in 0..(bits) {
        for i in 0..m {
            let mut new_bit = bit;
            for j in 0..n {
                new_bit |= a[i][j] << j;
            }
            chmin!(dp[i + 1][bit], dp[i][bit]);
            chmin!(dp[i + 1][new_bit], dp[i][bit] + 1);
        }
    }
    let ans = dp[m][bits - 1];
    println!("{}", if ans == 1000 { -1 } else { ans });
}
