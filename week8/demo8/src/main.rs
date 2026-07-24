/*
Concurrency:
Multiple program being executed concurrently

Concurrency vs Parallelism
Concurrency: Constly switching which program runs in cpu in a very fast time.
Parallelism: Have multiple cpu cores to run program at the same time.

Issues that could occur in concurrency:
- Data Race: Multiple threads could access the same data and modify it, then we con't know which result would be true
- Dead Lock: When multiple threads are waiting for other threads to release the lock
- Dangling Reference: Multiple threads could have reference pointing to the same data, when data is dropped in one thread, 
other thread would hold a dangling reference

How other languages solve concurrency issues:
C++: Mutex lock (Manully write lock and release lock for resource)
Java: Syncronized (Automatically handle concurrency resource)
Python: python GLI will make sure program is run in a single threaded situation

Thread Management:
- spawn a thread
- use thread scope to manage lifetime of each thread
- chunks method

How do we communicate between threads?
- Send & Sync Trait
- Channel: Sending object to different threads by ownership
    mpsc: multiple producer single consumer
- Arc: &T
- Mutex: *mut T
- Arc<Mutex<T>>: &mut T 
*/

// struct myStruct {   
//     a: i32,
//     b: bool
// }

use std::{println, sync::{Arc, Mutex, mpsc::channel}, thread};

fn main() {
    // println!("1");
    // println!("2");
    // println!("3");
    // println!("4");
    // println!("5");
    // println!("6");

    // spawning a thread
    {
        // let a = Arc::new(String::from("hello"));
        // let a_clone = Arc::clone(&a); // Not cloning the string, but instead cloning the pointer
        // let handle = thread::spawn(move || {
        //     println!("1 {}", a_clone); // a captured into closure through T
        //     println!("2");
        // });

        // let a_clone = a.clone();
        // let handle2 = thread::spawn(move || {
        //     println!("3 {}", a_clone);
        //     println!("4");
        // });

        // let a_clone = a.clone();
        // let handle3 = thread::spawn(move || {
        //     println!("5 {}", a_clone);
        //     println!("6");
        // });


        // // this thread may outlive the scope where we created it
        // // thread::spawn(|| {
        // //     println!("1");
        // //     println!("2");
        // // });


        // // we need to explicitly join the thread handle
        // handle.join().unwrap();
        // handle2.join().unwrap();
        // handle3.join().unwrap();
    }

    // Closure recaps
    // fn (type) -> Fn (traits) -> FnMut (traits) -> FnOnce (traits) -> 
    // Fn: &T
    // FnMut: &mut T
    // FnOnce: T

    // spawning a thread with scope
    {
        // let msg = String::from("hello");
        // thread::scope(|s| {
        //     s.spawn(|| {
        //         println!("1 {}", msg); // msg is being captured through &T
        //         println!("2");
        //     });

        //     s.spawn(|| {
        //         println!("3");
        //         println!("4");
        //     });

        //     s.spawn(|| {
        //         println!("5");
        //         println!("6");
        //     });

        //     // a.join().unwrap(); // can still manually call join

        //     // automatically join all the threads created in scope
        // });

        // println!("7 {}", msg);
    }

    // difference between scope spawn and thread::spawn
    {
        // // Demonstration of thread outliving its scope
        // let handle = {  // New scope starts
        //     let message = String::from("Hello from thread");
            
        //     // This thread can outlive the scope where `message` was created
        //     // because `message` is moved into the thread and has 'static lifetime

        //     // FnOnce => T for captured variable message
        //     // Fn => &T for captured variable message 
        //     // FnMut => &mut T for captured variable message

        //     thread::spawn(move || {
        //         // Simulate some work
        //         thread::sleep(std::time::Duration::from_secs(2));
        //         println!("{}", message);
        //     })
        // };  // Scope ends here, but thread continues running!
        
        // // Without this join, the program might end before the thread prints
        // handle.join().unwrap();
        
        
        // // But this works with thread::scope:
        // thread::scope(|s| {
        //     s.spawn(|| {
        //         thread::sleep(std::time::Duration::from_secs(1));
        //         println!("{}", &message);  // Works fine! Reference is valid for the scope
        //     });
            
        // }); // drop(scope), drop(message)
        // // joined at the end of the scope
    }

    // Chunks
    {
        // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // thread::scope(|s| {
        //     for chunk in v.chunks(3) {
        //         s.spawn(move ||  {
        //             println!("{:?}", chunk);
        //         });
        //     }
        // });
    }
    

    // Channel
    {
        // let (sender, receiver) = channel();

        // let sender1 = sender.clone();
        // let handle = thread::spawn(move || {
        //     let vals = vec![
        //         String::from("Hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("thread"),
        //     ];

        //     for val in vals {
        //         sender1.send(val).unwrap();
        //     }
        // });

        // let sender1 = sender.clone();
        // let handle2 = thread::spawn(move || {
        //     let vals = vec![
        //         String::from("Hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("thread"),
        //     ];

        //     for val in vals {
        //         sender1.send(val).unwrap();
        //     }
        // });


        // handle.join().unwrap();
        // handle2.join().unwrap();

        // // for msg in receiver {
        // //     println!("{}", msg);
        // // }
        // // println!("{}", receiver.recv().unwrap());
        // // println!("{}", receiver.recv().unwrap());
        // // println!("{}", receiver.recv().unwrap());
        // // println!("{}", receiver.recv().unwrap());
        // // println!("{}", receiver.recv().unwrap());
    }

    // Arc and Mutex
    {
        // // Create a shared counter using Arc and Mutex
        // // Arc<T> == &T
        // // Mutex<T> == mut T
        // // Arc<Mutex<T>> == &mut T
        // let counter = Arc::new(Mutex::new(0));
        
        // // Create multiple threads that will increment the counter
        // let mut handles = vec![];
        
        // // Spawn 3 threads
        // for i in 0..3 {
        //     let counter_reference = Arc::clone(&counter);
        //     // let counter_clone = counter.clone();
            
            
        //     let handle = thread::spawn(move || {
        //         // Lock the mutex to modify the counter
        //         // let mut num = counter.lock().unwrap();
        //         // *num += 1;
        //         let mut ptr = counter_reference.lock().unwrap();
        //         *ptr += 1;
        //         println!("Thread {} incremented counter to {:?}", i, *ptr);
        //     });
            
        //     handles.push(handle);
        // }
        
        // // Wait for all threads to complete
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        
        // // Print the final value
        // println!("Final counter value: {}", *counter.lock().unwrap());
    }

}
