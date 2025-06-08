// fn main() {
//     println!("Hello, world!");
// }

// // variable
// fn main_second() {
//     let x = 5;
//     let y: u32 = 10;
//     let z : f32= 3.14;
//     println!("The value of x is: {}", x);

//     let mut r : i8  = 20; // overflow

//     for i in 0..10000 {
//         r = r+100;
//     }

// }

// even checking snake cases in used

// fn main(){
//     println!("{}",is_even(4));

// }

// fn is_even(n: i32) -> bool {
//     if n % 2 == 0 {
//        return true
//     } else {
//        return false
//     }
// }


// fibonacci recurion

fn main() {
    let n = 10;
    println!("Fibonacci of {} is {}", n, fibonacci(n));
}
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
