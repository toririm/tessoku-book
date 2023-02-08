// A11 Binary Search 1
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38720718

use proconio::input;

fn find(list: &Vec<u32>, size: usize, y: u32) -> Option<usize> { // 0-based
    let mut l = 0;
    let mut r = size - 1;
    while l <= r {
        let m = (l + r) / 2;
        if list[m] < y {
            l = m + 1;
        } else if list[m] > y {
            r = m - 1;
        } else {
            return Option::Some(m);
        }
    }
    return Option::None;
}

fn main() {
    input!{
        n: usize,
        x: u32,
        a:[u32; n],
    }
    match find(&a, n, x) {
        Some(y) => println!("{}", y + 1), // 1-based
        None => panic!(),
    }
}
