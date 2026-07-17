fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply_function(f: TODO, x: i32) -> i32 {
    f(x)
}

fn call_with_fn(f: TODO, x: i32) -> i32 
{
    f(x)
}

fn call_with_fn_mut(mut f: TODO, x: i32) -> i32 {
    f(x)
}


fn call_with_fn_once(f: TODO, x: i32) -> String {
    f(x)
}

/*
fn: 

Fn: 
FnMut: 
FnOnce: 
*/


fn main() {
    //// what is the type of a function we can pass around?

    // let f = add_one;
    // println!("add_one(41) = {}", apply_function(f, 41));

    //// what if the callable needs to capture something from its environment?

    // let x = 3;
    // let add_x = |y| y + x; // captures x into closure
    // println!("closure result = {}", call_with_fn(add_x, 4));

    //// What if the closure needs to change something when it is called?

    // let mut total = 0;
    // let mut add_and_store = |value: i32| {
    //     total += value;
    //     total
    // };
    // println!("first call = {}", call_with_fn_mut(&mut add_and_store, 2));
    // println!("second call = {}", call_with_fn_mut(&mut add_and_store, 3));

    //// What if we want the closure to consume its captured value and can only be used once?

    // let message = String::from("Ada");
    // let consume_message = move |value: i32| {
    //     format!("{} says {value}", message);  
    // };
    // println!("{}", call_with_fn_once(consume_message, 7));

    // Finally, compare this with macros.
    // A macro does not work on values at runtime; it works on syntax and expands to code.
    
    // greet!("Bob");
    // print_many!("one" => 2.2 => 3.14);
}

