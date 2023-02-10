// A02 Linear Search
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38759168

use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        a:[usize; n],
    }
    let mut found: bool = false;
    for aa in &a {
        if *aa == x {
            found = true;
        }
    }
    if found {
        println!("Yes");
    } else {
        println!("No");
    }
}
