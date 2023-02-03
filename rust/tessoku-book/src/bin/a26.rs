// A26 Prime Check
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38566376

use proconio::input;

const MAX: usize = 30_00_00;

fn main() {
    input!{
        q: usize,
    }
    let mut is_prime =[true; MAX + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for n in 2..=MAX {
        if is_prime[n] {
            let mut not_prime = n + n;
            while not_prime <= MAX {
                is_prime[not_prime] = false;
                not_prime += n;
            }
        }
    }
    for _q in 0..q {
        input!{
            x: usize,
        }
        if is_prime[x] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
