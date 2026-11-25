// main.rs
// Date: Wed Nov 25 2026
// Educational and Practice Rust Programming Language Code

// Project: Generic in Rust Programming Language
// Goal: Finding max number from an array
// Dependency: Without any dependency

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

fn main() {
    println!("\n");

    let number_list = [19, 39, 97, 73, 15, 57, 59, 17, 91, 75];

    // -------------------------------------------------

    let result_one = get_max1(&number_list);
    if let Ok(val) = result_one {
        println!("Largest number in number_list is: {val}");
    } else if let Err(msg) = result_one {
        println!("--->>> {msg}");
    }

    // -------------------------------------------------

    let result_two = get_max2(&number_list);
    match result_two {
        Ok(result) => println!("max number is; {result}"),
        Err(em) => println!("--->>> {em}"),
    }

    println!("\nThe End ...\n");
}

#[allow(dead_code)]
fn get_max1<T>(num_list: &[T]) -> Result<&T, String>
where
    T: PartialOrd,
{
    let max_opt = num_list.first();
    let Some(mut max) = max_opt else {
        return Err(String::from(
            "first element of input array han no any value ...",
        ));
    };

    for item in num_list.get(1..).unwrap_or_default() {
        if item > max {
            max = item;
        }
    }
    Ok(max)
}

#[allow(dead_code)]
fn get_max2<T>(num_list: &[T]) -> Result<&T, String>
where
    T: PartialOrd,
{
    let Some((first, rest)) = num_list.split_first() else {
        return Err(String::from("the input slice is empty ..."));
    };

    let mut max = first;
    for item in rest {
        if item > max {
            max = item;
        }
    }
    Ok(max)
}
