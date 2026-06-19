&str
let a1 = String::from("Hello");
let a2 = "Hello"

let b: String = a1; // Moved a1's ownership to b
println!("{}", a1); // a1 no longer valid

let b: &str = a2; // Bitwise copying from a2 to b  
println!("{}", a2); // a2 is still valid

&[i32; 5] // has to give a fixed capacity. Size known at compile time

// [0, 1, 2, 0, 0]

let v: Vector<i32> = vec![]; // don't need to specify capacity. Size know at run time


//// Ownership & borrowing & lifetime system
/// 
/// Ensures that every value in Rust will only have one owner at a time
/// 
/// T: Ownership of the data, modify and read.
/// &T: multiple shared borrows, read only
/// &mut T: one borrow at a time, allow to mutate and read.
/// 
/// 
// struct Student {
//     zid: u32
// }

// let students = vec![...]

// struct Classroom {
//     students: &Vector
// }

// Vector<Classroom>



let a: String = Sting::from("Hello");

let b: char = (a as u8) as char;

let closure = |args| {
    println!("{}", a);
}


/// Functions:
/// 
/// FnOnce: Can only be called once, closure will owns the ownership of captured variables
/// 
/// FnMut: Takes a &mut T of the captured variables. 
/// Can be called multiple times 
/// 
/// Fn: Takes a &T of the captured variables. Can be called multiple times
/// 
/// fn: can treat functions as values, which means we can pass functions
/// to another function as args or move it into sturcts. etc






