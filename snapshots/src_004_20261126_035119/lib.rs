// lib.rs
// Date: Thu Nov 26 2026
// Educational and Practice Rust Programming Language Code

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

#[derive(Debug, Default, Clone, Copy)]
pub struct Calculate<T> {
    value_one: T,
    value_two: T,
}

impl<T: Clone + Copy + Default> Calculate<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            value_one: T::default(),
            value_two: T::default(),
        }
    }

    pub const fn set_value_one(&mut self, value_one: T) {
        self.value_one = value_one;
    }

    pub const fn set_value_two(&mut self, value_two: T) {
        self.value_two = value_two;
    }

    #[must_use]
    pub const fn value_one(&self) -> T {
        self.value_one
    }

    #[must_use]
    pub const fn value_two(&self) -> T {
        self.value_two
    }
}

impl Calculate<f32> {
    #[must_use]
    pub const fn add(&self) -> f32 {
        self.value_one + self.value_two
    }
}
