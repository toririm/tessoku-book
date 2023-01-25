// A04 Binary Representation 
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38326394

use proconio::input;

fn main() {
    input!{
        n: u32,
    }
    for i in (0..10).rev() {
        if n & (1 << i) == 0 {
            print!("0");
        } else {
            print!("1");
        }
    }
    print!("\n");
}
