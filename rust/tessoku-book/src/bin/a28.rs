// A28 Blackboard
// submission: https://atcoder.jp/contests/tessoku-book/submissions/38568646

use proconio::input;
use std::ops::{AddAssign, SubAssign, MulAssign};

const MODULO: u32 = 1_00_00;

struct Modulo {
    value: u32,
    modulo: u32,
}

impl AddAssign for Modulo {
    fn add_assign(&mut self, other: Self) {
        if self.modulo != other.modulo {
            panic!("modulo isn't the same");
        }
        *self = Self {
            value: (self.value + other.value) % self.modulo,
            modulo: self.modulo,
        };
    }
}

impl SubAssign for Modulo {
    fn sub_assign(&mut self, other: Self) {
        if self.modulo != other.modulo {
            panic!("modulo isn't the same");
        }
        *self = Self {
            value: (self.modulo + self.value - other.value) % self.modulo,
            modulo: self.modulo,
        };
    }
}

impl MulAssign for Modulo {
    fn mul_assign(&mut self, other: Self) {
        if self.modulo != other.modulo {
            panic!("modulo isn't the same");
        }
        *self = Self {
            value: (self.value * other.value) % self.modulo,
            modulo: self.modulo,
        };
    }
}

fn main() {
    input!{
        n: usize,
    }
    let mut b = Modulo{value: 0, modulo: MODULO};
    for _n in 0..n {
        input!{
            t: char,
            a: u32,
        }
        let aa = Modulo{value: a, modulo: MODULO};
        match t {
            '+' => b += aa,
            '-' => b -= aa,
            '*' => b *= aa,
            _ => panic!(),
        }
        println!("{}", b.value);
    }
}
