// A11 Binary Search 1
// submisson: https://atcoder.jp/contests/tessoku-book/submissions/38363095

use proconio::input;

// もっといい実装方法ある？
fn search(list: Vec<u32>, l: usize, r: usize, y: u32) -> usize {
    if l > r {
        panic!();
    }
    let m = (l + r) / 2;
    return if list[m] < y {
        search(list, m + 1, r, y)
    } else if list[m] > y {
        search(list, l, m - 1, y)
    } else {
        m
    };
}

fn main() {
    input!{
        n: usize,
        x: u32,
        a:[u32; n],
    }
    println!("{}", search(a, 0, n - 1, x) + 1);
}
