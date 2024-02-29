// GCD program from "Programming Rust: 2nd Ed".

use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n   
}

fn main() {
    let mut args: Vec<u64> = Vec::new();
    
    for arg in env::args().skip(1) {
        args.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if args.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d: u64 = args[0];
    for m in &args[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", args, d);
}
