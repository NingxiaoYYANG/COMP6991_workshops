
use bmp::{Image, Pixel};

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "something_else" {
        // Add more cases here!
    } else {
        eprintln!("The operation {operation} was not recognised!");
    }

    match operation.as_str() {
        "pixel" => {
            draw_pixel(path.as_str());
        },
        "something_else" => todo!(),
        _ => eprintln!("The operation {operation} was not recognised!"),
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}


































// // Rust basics

// fn main() {
//     /* 
//     Basic types in rust
//     integer: i8, i16, i32, u8, u16, u32, isize, usize
//     floating-point: f32, f64
//     boolean: bool
//     character: char
//     string: &str (only read)(Stack), String (read, write, update)(Heap)

//     Stack:
//     array: static memory size

//     Heap:
//     arrayList(Java): Dynamic memory size
//     */ 
    
//     // declare a variable
//     // let mut myVar = 5;
//     // myVar += 1; 


//     // if statement
//     // if 2 % 2 == 0 {
//     //     println!("Hello");
//     // } else if 2 % 2 == 1 {
//     //     println!("Hi");
//     // }

//     // for loop
    
//     // let vec = vec![1, 2, 3];
//     // for num in vec {
//     //     println!("curr num: {}", num);
//     // }

//     let first = foo(11);
//     let second= foo(9);

//     println!("First: {}, Second: {}", first, second);

// }

// // declare function
// fn foo(n: i32) -> i32 {
//     if n > 10 {
//         n
//     } else {
//         10
//     }
// } 































