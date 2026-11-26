// main.rs
// Date: Thu Nov 26 2026
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

use rust_generic::Calculate;

fn main() {
    println!("\n");

    let mut calc_one: Calculate<i32> = Calculate::new();
    calc_one.set_value_one(391);
    calc_one.set_value_two(793);

    println!("calc_one is: {calc_one:?}");
    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    // -----------------------------------------

    let mut calc_two: Calculate<f32> = Calculate::new();
    calc_two.set_value_one(1_379.973_1);
    calc_two.set_value_two(9_731.138);

    // add() method appears only for calc_two
    // because calc_two holds `f32` data type.
    let add_result = calc_two.add();

    println!("calc_two is: {calc_two:?}");
    println!("result of add is: {add_result}");

    println!("\nThe End ...\n");
}
