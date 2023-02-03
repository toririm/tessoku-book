// A29 Power
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38575451

use proconio::input;

const MODULO: u64 = 1_000_000_007;

fn power(x: u64, y: u64) -> u64 {
    let mut ret = 1;
    let mut tmp = x;
    for i in 0..30 {
        if y & (1 << i) > 0 {
            ret *= tmp;
            ret %= MODULO;
        }
        tmp = tmp * tmp;
        tmp %= MODULO;
    }
    return ret;
}

fn main() {
    input!{
        a: u64,
        b: u64,
    }
    println!("{}", power(a, b));
}
