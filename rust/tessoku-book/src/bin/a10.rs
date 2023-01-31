// A10 Resort Hotel
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38500737

use proconio::input;
use std::cmp::max;

fn main() {
    input!{
        n: usize,
        a:[u32; n],
        d: usize,
    }
    let mut max_left = a.to_vec();
    let mut max_right = a.to_vec();
    for i in 0..(n - 1) {
        max_left[i + 1] = max(max_left[i + 1], max_left[i]);
    }
    for i in (1..n).rev() {
        max_right[i - 1] = max(max_right[i - 1], max_right[i]);
    }
    for _dd in 0..d {
        input!{
            mut l: usize,
            mut r: usize,
        }
        l -= 1;
        r -= 1;
        println!("{}", max(max_left[l - 1], max_right[r + 1]));
    }
}
