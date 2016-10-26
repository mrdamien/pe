extern crate num;
extern crate time;
use std::ops::{Add, Mul};
use num::bigint::*;
use num::traits::*;
use std::collections::HashMap;


fn pow (a:u64, b:u64) -> BigInt {
    let mut x = a.to_bigint().unwrap();
    let mut i = 1;
    while i < b {
        {
            let t = x * a.to_bigint().unwrap();
            x = t;
        }
        i += 1;
    }
    x
}

fn main () {
    let start = time::precise_time_ns();
    let mut results = Box::new(HashMap::new());

    let mut a:u64 = 2;
    let mut b:u64 = 2;
    let mut counter = 0;
    loop {
        b = 2;
        loop {
            let t:BigInt = pow(a, b);
            //println!("{}^{} = {}", a, b, t.to_str_radix(10));
            results.entry(t).or_insert(true); // format!("{}", t.to_str_radix(36))
            counter += 1;
            b += 1;
            if b == 101 {
                break;
            }
        }
        a += 1;
        if a == 101 {
            break;
        }
    }

    println!("{}/{} unique results", results.len(), counter);
    let end = time::precise_time_ns();

    println!("took {} ns", end - start);
}
