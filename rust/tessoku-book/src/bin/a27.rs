// A27 Calculate GCD
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38566660

use proconio::input;

fn gcd(x: u32, y: u32) -> u32 {
    if x % y == 0 {
        y
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input!{
        a: u32,
        b: u32,
    }
    println!("{}", gcd(a, b));
}
