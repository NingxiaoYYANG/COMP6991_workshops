// Rust basics

fn main() {
    /* 
    Basic types in rust
    integer: i32, i8, i16, u32, u8, u16, isize, usize
    floating-point: f32, f64
    boolean: bool
    character: char
    string: &str(read only), String(read, write, update)

    Stack: &str (Static size. fixed)

    Heap: String (Dynaimic size. Modifiable)
    */ 
    
    // declare a variable
    // let mut my_var: i32 = 42;
    // my_var += 1;
    // println!("{}", my_var.abs());


    // if statement
    // let is_even = if 2 % 2 == 0 {
    //     true
    // } else {
    //     false
    // };

    

    // for loop;
    // for (int i = 0; i < n; i++) {
    //     v[i]
    // }

    // let v = vec![1, 2, 3];

    // for num in v {
    //     println!("{}", num);
    // }

    // println!("{}", is_even(5));

}

// declare function
// all functions are expressions
// fn is_even(n: i32) -> bool {
//     if n % 2 == 0 {
//         return true;
//     }
//     false
// }










































// use bmp::{Image, Pixel};

// fn main() {
//     let path = std::env::args().nth(1).expect("You must provide a path.");
//     let operation = std::env::args().nth(2).expect("You must provide an operation.");

//     if operation.as_str() == "pixel" {
//         draw_pixel(path.as_str());
//     } else if operation.as_str() == "something_else" {
//         // Add more cases here!
//     } else {
//         eprintln!("The operation {operation} was not recognised!");
//     }

//     match operation.as_str() {
//         "pixel" => {
//             draw_pixel(path.as_str());  
//         },
//         "something_else" => todo!(),
//         _ => eprintln!("The operation {operation} was not recognised!"),
//     }
// }

// fn draw_pixel(path: &str) {
//     let mut image = Image::new(100, 100);
//     image.set_pixel(50, 50, Pixel::new(255, 255, 255));
//     image.save(path).expect("This should save correctly.");
// }





























































