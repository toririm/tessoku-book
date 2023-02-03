// A24 LIS
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38565105

use proconio::input;
use std::cmp::max;

fn binary_search(x: &Vec<usize>, y: usize) -> usize {
    let mut l = 0;
    let mut r = x.len() - 1;
    while l < r {
        let m = (l + r) / 2;
        if x[m] < y {
            l = m + 1;
        }
        if x[m] >= y {
            r = m;
        }
    }
    return l;
}

fn main() {
    input!{
        n: usize,
        a:[usize; n],
    }
    let mut dp = vec![10_usize.pow(6); n + 1];
    dp[0] = 0;
    let mut ans = 0;
    for i in 0..n {
        let j = binary_search(&dp, a[i]);
        ans = max(ans, j);
        dp[j] = a[i];
    }
    println!("{}", ans);
}
