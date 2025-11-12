#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufRead,BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;




// fn main() {
//     println!("what is your name?");
//     let mut name = String::new();
//     let greet = "nice to meet you";
//     io::stdin().read_line(&mut name).expect("failed to read line");

//     println!("Hello, {}! {}", name.trim_end(), greet);

//     println!("Hello, world!");
// }


fn main() {
    const ONE_MILLION: u32 = 1_000_000;
    println!("one million is written as {}", ONE_MILLION);

    let age = "47";
    let mut age: u32 = age.trim().parse().expect("age wasn't assigned a number");
    age += 1;
    println!("next year, you will be {}", age);
}

