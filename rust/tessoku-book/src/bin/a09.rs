// A09 Winter in ALGO Kingdom
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38345197

use proconio::input;

fn main() {
    input!{
        h: usize,
        w: usize,
        n: usize,
    }
    let mut x = vec![vec![0i32; w + 1]; h + 1];
    for _t in 0..n {
        input!{
            mut a: usize,
            mut b: usize,
            c: usize,
            d: usize,
        }
        a -= 1;
        b -= 1;
        x[c][d] += 1;
        x[a][d] -= 1;
        x[c][b] -= 1;
        x[a][b] += 1;
    }
    for i in 0..h {
        for j in 0..w {
            x[i][j + 1] += x[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            x[i + 1][j] += x[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}{}", x[i][j], if j == w - 1 { "\n" } else { " " });
        }
    }
}
