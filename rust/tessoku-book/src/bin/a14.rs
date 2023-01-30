// A14 Four Boxes
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38477524

use proconio::input;

fn main() {
    input!{
        n: usize,
        k: u32,
        abcd:[[u32; n]; 4],
    }
    let mut p = vec![0_u32; n * n];
    let mut q = vec![0_u32; n * n];
    for i in 0..n {
        for j in 0..n {
            p[i * n + j] = abcd[0][i] + abcd[1][j];
        }
    }
    for i in 0..n {
        for j in 0..n {
            q[i * n + j] = abcd[2][i] + abcd[3][j];
        }
    }
    q.sort();
    for pp in p {
        let index = q.binary_search(&(k - pp));
        match index {
            Ok(_) => {println!("Yes"); return;},
            Err(_) => {},
        }
    }
    println!("No");
}
