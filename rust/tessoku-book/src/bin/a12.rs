// A12 Printer
// submision: https://atcoder.jp/contests/tessoku-book/submissions/38445121

use proconio::input;

fn main() {
    input!{
        n: usize,
        k: u64,
        a:[u64; n],
    }
    let mut l: u64 = 1;
    let mut r: u64 = 10_u64.pow(9);
    while l < r {
        let m = (l + r) / 2;
        let is_fulfilled: bool = {
            let mut sum = 0;
            for aa in &a {
                sum += m / aa;
            }
            sum >= k
        };
        if is_fulfilled {
            r = m;
        } else {
            l = m + 1;
        }
    }
    println!("{}", l); // l == r
}
