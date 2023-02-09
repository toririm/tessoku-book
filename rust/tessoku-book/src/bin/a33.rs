// A33 Game 2
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38740381

use proconio::input;

fn main() {
    input!{
        n: usize,
        a:[u32; n],
    }
    let mut xor_sum = a[0];
    for i in 1..n {
        xor_sum ^= a[i];
    }
    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
