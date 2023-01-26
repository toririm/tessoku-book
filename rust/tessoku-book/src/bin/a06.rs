// A06 How Many Guests?
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38344081

use proconio::input;

fn main() {
    input!{
        n: usize,
        q: u32,
        a:[u32; n]
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    for _j in 0..q {
        input!{
            l: usize,
            r: usize,
        }
        println!("{}", b[r] - b[l - 1]);
    }
}
