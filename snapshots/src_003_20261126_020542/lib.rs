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

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default)]
pub struct Calculate<T, U> {
    x: T,
    y: U,
}

impl<T, U> Calculate<T, U>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    T: Clone + Copy + Default,
    U: Add<Output = U> + Sub<Output = U> + Mul<Output = U> + Div<Output = U>,
    U: Clone + Copy + Default + TryFrom<T>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            x: T::default(),
            y: U::default(),
        }
    }

    #[must_use]
    pub const fn new_from(x: T, y: U) -> Self {
        Self { x, y }
    }

    pub const fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub const fn set_y(&mut self, y: U) {
        self.y = y;
    }

    #[must_use]
    pub const fn get_x(&self) -> T {
        self.x
    }

    #[must_use]
    pub const fn get_y(&self) -> U {
        self.y
    }

    /// # Errors
    /// Returns `Result<U, <U as TryFrom<T>>::Error>`
    pub fn add(&self) -> Result<U, <U as TryFrom<T>>::Error> {
        let row = U::try_from(self.x)?;
        Ok(row + self.y)
    }

    /// # Errors
    /// Returns `Result<U, <U as TryFrom<T>>::Error>`
    pub fn sub(&self) -> Result<U, <U as TryFrom<T>>::Error> {
        let row = U::try_from(self.x)?;
        Ok(row - self.y)
    }

    /// # Errors
    /// Returns `Result<U, <U as TryFrom<T>>::Error>`
    pub fn mul(&self) -> Result<U, <U as TryFrom<T>>::Error> {
        let row = U::try_from(self.x)?;
        Ok(row * self.y)
    }

    /// # Errors
    /// Returns `Result<U, <U as TryFrom<T>>::Error>`
    pub fn div(&self) -> Result<U, <U as TryFrom<T>>::Error> {
        let row = U::try_from(self.x)?;
        Ok(row / self.y)
    }
}
