// A08 Two Dimensional Sum
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38344342

use proconio::input;

fn main() {
    input!{
        h: usize,
        w: usize,
        x:[[u32; w]; h],
        q: usize,
    }
    let mut y = vec![vec![0u32; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            y[i + 1][j + 1] = y[i + 1][j] + x[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            y[i + 1][j + 1] += y[i][j + 1];
        }
    }
    for _qq in 0..q {
        input!{
            mut a: usize,
            mut b: usize,
            c: usize,
            d: usize,
        }
        a -= 1;
        b -= 1;
        println!("{}", y[c][d] - y[a][d] - y[c][b] + y[a][b]);
    }
}
