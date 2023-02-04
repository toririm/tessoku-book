// A30 Combination
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38586409

use proconio::input;

const MODULO: u64 = 1_000_000_007;

fn factorial(n: u64) -> u64 {
    let mut ret = 1;
    for i in 1..=n {
        ret *= i;
        ret %= MODULO;
    }
    ret
}

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

fn inverse(b: u64) -> u64 {
    let bb = b % MODULO;
    power(bb, MODULO - 2)
}

fn main() {
    input!{
        n: u64,
        r: u64,
    }
    println!("{}", (factorial(n) * inverse(factorial(r) * factorial(n - r))) % MODULO);
}
