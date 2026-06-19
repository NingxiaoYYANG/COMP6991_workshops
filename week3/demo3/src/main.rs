// #[derive(Debug, Copy, Clone)]
// // struct Point {
// //     x: String,
// //     y: i32
// // }

// Clone: deep clone, more expensive operation
// fn foo(s: String) -> String {

// }

// bit by bit copying the reference from outside &String to s
// fn foo(s: &String) {

// }

// fn foo(s: &mut String) {

// }

fn main() {
    // Ownership & Borrowing

    // Heap: String, Vector, HashMap (Dynamic Sizes)
    // Stack: i32, u32, bool, usize, Array, &str
    let mut a = String::from("Hello");
    // move
    // let b = a; // Move by default
    // let bb = aa; // Copy bit by bit
    // println!("{}", a);

    // clone
    // let b = a.clone(); // Clone

    // borrow: Exclusive borrow, shared borrow

    
    
    // &: Immutable reference, shared borrow
    // let b = &a; // borrowing the data from a, but immutable
    // // b.push('!');
    // println!("{}", b);
    // let c = &a; // borrowing the data from a, but immutable
    // // b.push('!');
    // println!("{}", b);
    // let d = b; // borrowing the data from a, but immutable
    // // b.push('!');
    // println!("{}", d);

    // // &mut: Mutable reference, exclusive borrow
    // let b = &mut a; // borrowing the data from a, mutable 
    // // Cannot borrow because multiple references are trying to mutate same data
    
    // b.push('!');
    // // let c = &mut a; 
    // // c.push('?');
    


    // // Three iter types: iter(), iter_mut(), into_iter()
    // iter() -> & T
    // iter_mut() -> &mut T
    // into_iter() -> T

    // let mut vec = vec![1, 2, 3, 4];
    // for num in vec.iter_mut() {
    //     // * : deference
    //     *num += 1;
    //     println!("{}", num);
    // }

    // println!("{:?}", vec);
    


    // common slices in rust : &[T], &mut [T], &str
    // slices are a range of borrowed data through reference from a contiguous data type
    // contiguous data types: string literal, vec, array
    // let mut vec = vec![1, 2, 3, 4];
    // let slice:[i32; 2] = vec[1..=2];
    // let arr = [1, 2, 3, 4]; // [T; capacity] 
    // println!("{:?}", slice);

    // slice[1] = 100;
    // println!("{:?}", vec);

    // let a = "hello"; // slice from string literal
    

    // why do we need slices: slices give you special ability called unpacking
    
    // difference between 'slice' and 'reference'
    // slice: slices are a range of borrowed data through reference from a contiguous data type
    // reference: A blices are a range of borrowed data through reference from a contiguous data typeorrow from a single element 
    

}
