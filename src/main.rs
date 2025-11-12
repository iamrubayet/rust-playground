#![allow(unused)]

use std::{io, string};
use rand::Rng;
use std::io::{Write,BufRead,BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;


// std library example

// fn main() {
//     println!("what is your name?");
//     let mut name = String::new();
//     let greet = "nice to meet you";
//     io::stdin().read_line(&mut name).expect("failed to read line");

//     println!("Hello, {}! {}", name.trim_end(), greet);

//     println!("Hello, world!");
// }

// shadowing example
// fn main() {
//     const ONE_MILLION: u32 = 1_000_000;
//     println!("one million is written as {}", ONE_MILLION);

//     let age = "47";
//     let mut age: u32 = age.trim().parse().expect("age wasn't assigned a number");
//     age += 1;
//     println!("next year, you will be {}", age);
// }

// data types example

// fn main(){
//     let is_true : bool = true;
//     let a_float : f64 = 1.0; // regular annotation
//     let an_integer = 5i32; // suffix annotation
//     let default_float = 3.0; // f64
//     let default_integer = 7; // i32
//     let mut mutable = 12; // mutable i32
//     mutable = 21;

// }

// math
// fn main(){
//     let num_5: u32 = 5;
//     let num_4: u32 = 4;
//     let sum = num_5 + num_4;
//     let difference = num_5 - num_4;
//     let product = num_5 * num_4;
//     let quotient = num_5 / num_4;
//     let remainder = num_5 % num_4;
//     println!("sum: {}", sum);
//     println!("difference: {}", difference);
//     println!("product: {}", product);
//     println!("quotient: {}", quotient);
//     println!("remainder: {}", remainder);
// }

// random number generator example
// fn main(){
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {}", secret_number);
// }

// if else 
// fn main()
// {
//     let age = 8;

//     if (age >= 1) && (age <= 18){
//         println!("you get a child discount");
//     }
//     else if (age >= 19) && (age <= 65){
//         println!("you pay full price");
//     }
//     else{
//         println!("you get a senior citizen discount");
//     }

//     let age = 8;

//     let can_vote = if age >= 18 { true } else { false };

//     let age2 = 20;

//     match age2 {
//         1..=18 => println!("you get a child discount"),
//         19..=65 => println!("you pay full price"),
//         _ => println!("you get a senior citizen discount"),
//     }


//     let my_age = 47;

//     let voting_age = 18;

//     match my_age.cmp(&voting_age) {
//         Ordering::Less => println!("you are not old enough to vote"),
//         Ordering::Greater => println!("you are old enough to vote"),
//         Ordering::Equal => println!("you are exactly old enough to vote"),
//     }

// }


// array 

// fn main(){
//     let array1 = [1,2,3,4,5];
//     let array2: [i32;5] = [1,2,3,4,5];
//     let array3 = [3;5]; // all elements are 3 and length is 5

//     println!("first element of array1: {}", array1[0]);
//     println!("second element of array2: {}", array2[1]);
//     println!("third element of array3: {}", array3[2]);

//     // length of array
//     println!("length of array1: {}", array1.len());

//     let mut loop_index = 0; 

//     // looping through array using while
//     loop {
//         if array1[loop_index] % 2 == 0 {
//             println!("{} is even", array1[loop_index]);
//         } else {
//             println!("{} is odd", array1[loop_index]);
//         }
//         loop_index += 1;
//         if loop_index >= array1.len() {
//             break;
//         }
//     }


//     while loop_index < array2.len() {
//         if array2[loop_index] % 2 == 0 {
//             println!("{} is even", array2[loop_index]);
//         } else {
//             println!("{} is odd", array2[loop_index]);
//         }
//         loop_index += 1;
//     }



//     for val in array3.iter() {
//         if *val % 2 == 0 {
//             println!("{} is even", *val);
//         } else {
//             println!("{} is odd", *val);
//         }
//     }

// }



// tuples

// fn main() {

//     let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

//     println!("first value: {}", my_tuple.0);
//     println!("second value: {}", my_tuple.1);
//     println!("third value: {}", my_tuple.2);



//     let (x, y, z) = my_tuple;

//     println!("x: {}", x);
//     println!("y: {}", y);
//     println!("z: {}", z);

// }


// strings 
// fn main() {
//   let mut str1 = String::new();

//    str1.push('a');
//    str1.push_str("bcdef");

//    for word in str1.split_whitespace() {
//        println!("{}", word);
//    }

//    let str2 = str1.replace("bc", "12");

//    let str3 = string::String::from("hello world");

//    let mut v1 : Vec<char> = str3.chars().collect();
//     v1.sort();
//     v1.dedup();
//     for c in v1 {
//          println!("{}", c); 
//     }

//     let str4 : &str = "hello world";

//     let mut str5 : String = str4.to_string();

//     println!("str5: {}", str5);

//     let bytes = str5.as_bytes();

//     let str6 = &str5[0..5];

//     str5.clear();


//     let str7 = String::from("hello");
//     let str8 = String::from("world");

//     let str9 = str7 + &str8;

//     for char in str9.chars() {
//         println!("{}", char);
//     }

//     for char in str9.bytes() {
//         println!("{}", char);
//     }





// }


// casting

// fn main(){
//     let int_num: i32 = 65;
//     let float_num: f64 = int_num as f64;
//     let char_num: char = int_num as u8 as char;

//     println!("int_num: {}", int_num);
//     println!("float_num: {}", float_num);
//     println!("char_num: {}", char_num);
// }


// enums 

// fn main() {
//     enum Day {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday,
//     }

//     let today: Day = Day::Wednesday;

//     match today {
//         Day::Monday => println!("Today is Monday"),
//         Day::Tuesday => println!("Today is Tuesday"),
//         Day::Wednesday => println!("Today is Wednesday"),
//         Day::Thursday => println!("Today is Thursday"),
//         Day::Friday => println!("Today is Friday"),
//         Day::Saturday => println!("Today is Saturday"),
//         Day::Sunday => println!("Today is Sunday"),
//     }
// }


// vectors

// fn main() {
//     let mut vec1: Vec<i32> = Vec::new();

//     vec1.push(1);
//     vec1.push(2);
//     vec1.push(3);
//     vec1.push(4);
//     vec1.push(5);

//     for val in vec1.iter() {
//         println!("{}", val);
//     }

//     for i in 0..vec1.len() {
//         println!("vec1[{}] = {}", i, vec1[i]);
//     }

//     vec1.pop();

//     for val in &vec1 {
//         println!("{}", val);
//     }

//     for val in vec1.iter_mut() {
//         *val *= 2;
//     }


// }



// functions 


// fn sum (a: i32, b: i32) -> i32 {
//     return a + b;
// }


// fn sum_list (list: &[i32]) -> i32 {
//     let mut total = 0;
//     for &val in list.iter() {
//         total += val;
//     }
//     total
// }


// fn main(){
//     let result = sum(5, 10);
//     println!("The sum is: {}", result);


//     let newlist = vec![1,2,3,4,5];
//     let doubledlist: Vec<i32> = newlist.iter().map(|x| x * 2).collect();
//     println!("Doubled list: {:?}", doubledlist);

//     let total = sum_list(&newlist);
//     println!("The total sum of the list is: {}", total);

// }



fn get_sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}


// generics
fn main() {

}