// A32 Game 1
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38678056

use proconio::input;

fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
    }
    let mut win = vec![false; n + 1];
    for i in 0..=n {
        if win[i] == false {
            if i + a <= n {
                win[i + a] = true;
            }
            if i + b <= n {
                win[i + b] = true;
            }
        }
    }
    if win[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
