// A13 Close Pairs
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38449143

use proconio::input;

fn main() {
    input!{
        n: usize,
        k: u32,
        a:[u32; n],
    }
    let mut ans: u64 = 0;
    let mut r = 1;
    for l in 0..(n - 1) {
        while r < n && a[r] - a[l] <= k {
            r += 1;
        } // このブロック終了時点でrが1-basedになるようにする
        ans += (r - (l + 1)) as u64; // 1-based
    }
    println!("{}", ans);
}
