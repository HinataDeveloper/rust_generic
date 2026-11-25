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

    let calc: Calculate<i32, f64> = Calculate::new_from(12, 36.963);

    let add = calc.add();
    let sub = calc.sub();
    let mul = calc.mul();
    let div = calc.div();

    match add {
        Ok(result) => println!("result of add is: {result}"),
        Err(err) => println!("--->>> {err}"),
    }

    match sub {
        Ok(result) => println!("result of subtract is: {result}"),
        Err(err) => println!("--->>> {err}"),
    }

    match mul {
        Ok(result) => println!("result of multiply is: {result}"),
        Err(err) => println!("--->>> {err}"),
    }

    match div {
        Ok(result) => println!("result of divid is: {result}"),
        Err(err) => println!("--->>> {err}"),
    }

    println!("\nThe End ...\n");
}
