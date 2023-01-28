// A05 Three Cards
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38365578

use proconio::input;

fn main() {
    input!{
        n: i32,
        k: i32,
    }
    let mut ans: u32 = 0;
    for r in 1..=n  {
        for b in 1..=n {
            let w = k - r - b;
            if 1 <= w && w <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
