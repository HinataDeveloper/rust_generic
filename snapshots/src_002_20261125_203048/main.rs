// main.rs
// Date: Wed Nov 25 2026
// Educational and Practice Rust Programming Language Code

// Project: Generic in Rust Programming Language
// Goal: ...
// Dependency: ...

// Kernel Version: 7.3.4-300.fc45.x86_64

// rustc 1.102.0-nightly (20fe935d5 2026-11-23)
// binary: rustc
// commit-hash: 20fe935d50367c0ff8872c2ebfa050c5c99d9201
// commit-date: 2026-11-23
// host: x86_64-unknown-linux-gnu
// release: 1.102.0-nightly
// LLVM version: 23.1.6

// cargo 1.102.0-nightly (ae8f06eb4 2026-11-21)
// release: 1.102.0-nightly
// commit-hash: ae8f06eb41b8c4a534d0f280f1b9d287f45ead73
// commit-date: 2026-11-21
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 45.0.0 [64-bit]

use std::ops::{Add, Div, Mul, Sub};

fn main() {
    println!("\n");

    let mut calc_one: Calc<u64> = Calc::new();

    calc_one.set_value_one(791);
    calc_one.set_value_two(391);

    println!("value_one is: {}", calc_one.value_one());
    println!("value_two is: {}", calc_one.value_two());

    let add = calc_one.add();
    let sub = calc_one.sub();
    let mul = calc_one.mul();
    let div = calc_one.div();

    println!("result of add is: {add}");
    println!("result of sub is: {sub}");
    println!("result of mul is: {mul}");
    println!("result of div is: {div}");

    println!("\nThe End ...\n");
}

struct Calc<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    T: Clone + Copy + Default,
{
    value_one: T,
    value_two: T,
}

impl<T> Calc<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    T: Clone + Copy + Default,
{
    #[must_use]
    fn new() -> Self {
        Self {
            value_one: T::default(),
            value_two: T::default(),
        }
    }

    #[allow(dead_code)]
    #[must_use]
    const fn new_from(v_one: T, v_two: T) -> Self {
        Self {
            value_one: v_one,
            value_two: v_two,
        }
    }

    const fn set_value_one(&mut self, data: T) {
        self.value_one = data;
    }

    const fn set_value_two(&mut self, data: T) {
        self.value_two = data;
    }

    const fn value_one(&self) -> T {
        self.value_one
    }

    const fn value_two(&self) -> T {
        self.value_two
    }

    #[must_use]
    fn add(&self) -> T {
        self.value_one + self.value_two
    }

    #[must_use]
    fn sub(&self) -> T {
        self.value_one - self.value_two
    }

    #[must_use]
    fn mul(&self) -> T {
        self.value_one * self.value_two
    }

    #[must_use]
    fn div(&self) -> T {
        self.value_one / self.value_two
    }
}
