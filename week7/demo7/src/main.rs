macro_rules! greet {
    ($x:expr) => {
        println!("{}", $x);
    };
}

macro_rules! print_many {
    ($($x:expr),* $(,)?) => {
        $(
            println!("{}", $x);
        )*
    };
}

fn double_one(x: i32) -> i32 {
    x * 2
}

fn apply_function(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn call_with_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 
{
    f(x)
}

fn call_with_fn_mut<F: FnMut(i32) -> i32> (f: &mut F, x: i32) -> i32 {
    f(x)
}


fn call_with_fn_once(f: impl FnOnce(i32) -> String, x: i32) -> String {
    f(x)
}



/*
fn: Function pointers, concret type

closure: 
Function without name, can capture environment variables

Closure Traits:
Fn: Closure that captures environment through &T, Can called multiple times
FnMut: Closure that captures environment through &mut T, Can called multiple times
FnOnce: Closure that captures environment through T, Can only called once

Metaprogramming:
Write code that manipulate or generates code

Macros:

Declartive Macro: ! (println!, dbg!, vec!)
A pattern matching way to match code token fragments, then produce code expressions

Procedure Macro: #[] (#[derive(...)], #[cfg(test)], #[attribute])

*/


fn main() {
    //// what is the type of a function we can pass around?

    // let f = add_one;
    // // f(3);
    // println!("add_one(41) = {}", apply_function(double_one, 41));

    //// what if the callable needs to capture something from its environment?

    // let v = vec![1, 2, 3];

    // println!("{:?}", v.iter().map(|x| x * 2).collect::<Vec<i32>>());    

    // let x = 3 + 4 * 2;
    // let add_x = |y| {
    //     y + x
    // }; // captures x into closure

    // println!("{}", apply_function(add_x, 4));
    // println!("closure result = {}", call_with_fn(add_one, 4));

    //// What if the closure needs to change something when it is called?

    // let mut total = 0;
    // let mut add_and_store = |value: i32| {
    //     total += value;
    //     total
    // };
    // // let my_var = total; //
    // println!("first call = {}", call_with_fn_mut(&mut add_and_store, 2));
    // println!("second call = {}", call_with_fn_mut(&mut add_and_store, 3));

    //// What if we want the closure to consume its captured value and can only be used once?

    // let message = String::from("Ada");
    // let consume_message = move |value: i32| {
    //     format!("{} says {value}", message);
    //     drop(message);
    //     // message += "hello";
    //     "Hello".to_string()
    // };
    // consume_message(7);
    // consume_message(7);
    // println!("{}", call_with_fn_once(&consume_message, 7));
    // dbg!(message);
    // println!("{}", call_with_fn_once(&consume_message, 7));

    // Finally, compare this with macros.
    // A macro does not work on values at runtime; it works on syntax and expands to code.
    
    // greet!(3 + 4 + 5);
    // print_many!("one", 2, 3.14,);
}

/*
Fragments Type:
ident: identity, the name of variable or function
expr: expression
ty: type
block: block expression, expr in {}
literal: _
tt: 

*/



// fn add_one(x: i32) -> i32 {
//     x + 1
// }