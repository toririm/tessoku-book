// A31 Divisors
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38638916

use proconio::input;

fn divisor(max: u64, a: u64) -> u64 {
    max / a
}

fn main() {
    input!{
        n: u64,
    }
    println!("{}", divisor(n, 3) + divisor(n, 5) - divisor(n, 15));
}
