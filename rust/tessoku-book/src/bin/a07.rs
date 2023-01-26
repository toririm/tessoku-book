// A07 Event Attendance
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38344141

use proconio::input;

fn main() {
    input!{
        d: usize,
        n: usize,
    }
    let mut a = vec![0; d + 1];
    for _i in 0..n {
        input!{
            l: usize,
            r: usize,
        }
        a[l - 1] += 1;
        a[r] -= 1;
    }
    for j in 0..d {
        a[j + 1] += a[j];
        println!("{}", a[j]);
    }
}
