fn main() {
    println!("Hello, world!");
}

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

// fn main() {
//     let n = 10;
//     println!("Fibonacci of {} is {}", n, fibonacci(n));
// }
// fn fibonacci(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fibonacci(n - 1) + fibonacci(n - 2);
//     }
// }

// fn fib(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     }
//     let mut a = 0;
//     let mut b = 1;
//     for _ in 2..=n {
//         let temp = b;
//         b = a + b;
//         a = temp;
//     }
//     b
// }

// fn main() {
//     let n = 10;
//     println!("Fibonacci of {} is {}", n, fib(n));
// }


// fn fib(n:u32)->u32 {
//     let mut first =0 ;
//     let mut second = 1;
//     if n == 0 {
//         return first;
//     }
//     if n == 1{
//         return second;
//     }

//     for _ in 0..(n-1){
//         let temp = second;
//         second = first + second;
//         first = temp;
//     }
//     return second;
// }


// string manipulation
// fn get_string_lenght(s: &str) -> usize {
//     s.chars().count()
// }
// fn main() {
//     let my_string = String::from("Hello, Rust!");
//     let length = get_string_lenght(&my_string);
//     println!("The length of the string is: {}", length);
// }


// struct 

// struct User {
//     name: String,
//     age: u32,
// }

// struct Rect{
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     // static method not attached to an instance
//     fn debug(){
//         println!("This is a rectangle");
//     }

// }

// fn main(){
//     let user1 = User {
//         name: String::from("Alice"),
//         age: 30,
//     };

//     let user2 = User {
//         name: String::from("Bob"),
//         age: 25,
//     };

//     println!("User 1: Name: {}, Age: {}", user1.name, user1.age);
//     println!("User 2: Name: {}, Age: {}", user2.name, user2.age);

//     let rect = Rect {
//         width: 10,
//         height: 5,
//     };
//     println!("Rectangle area: {}", rect.area());
//     Rect::debug();
// }

// enum example

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// fn main() {
//     let my_direction = Direction::Up;
//     let my_new_direction = my_direction;
//     move_enum(my_new_direction);


// }

// fn move_enum(direction: Direction) {

   
// }

// pattern matching with enum


// enum Shape {
//     Circle(f64), // radius
//     Rectangle(f64, f64), // width, height
// }
// fn main() {
//     let my_shape = Shape::Circle(5.0);
//     let area = calculate_area(my_shape);
//     println!("The area of the shape is: {}", area);
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }






 


