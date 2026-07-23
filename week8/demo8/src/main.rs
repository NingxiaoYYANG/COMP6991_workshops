/*
Concurrency:

Concurrency vs Parallelism

Issues that could occur in concurrency:
- Data Race
- Dead Lock
- Dangling Reference


How other languages solve concurrency issues:
C++:
Java:
Python:

Thread Management:
- spawn a thread
- use thread scope to manage lifetime of each thread
- chunks method

How do we communicate between threads?
- Send & Sync Trait
- Channel
    mpsc
- Arc
- Mutex
*/

fn main() {
    // println!("1");
    // println!("2");
    // println!("3");
    // println!("4");
    // println!("5");
    // println!("6");

    // spawning a thread
    {
        // let handle = thread::spawn(|| {
        //     println!("1");
        //     println!("2");
        // });

        // let handle2 = thread::spawn(|| {
        //     println!("3");
        //     println!("4");
        // });

        // let handle3 = thread::spawn(|| {
        //     println!("5");
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
        // // let msg = String::from("hello");
        // thread::scope(|s| {
        //     let a = s.spawn(|| {
        //         println!("1");
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

        // println!("7");
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

    // Channel
    {
        // let (sender, receiver) = channel();

        // let handle = thread::spawn(move || {
        //     let vals = vec![
        //         String::from("Hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("thread"),
        //     ];

        //     for val in vals {
        //         sender_clone.send(val).unwrap();
        //     }
        // });



        // handle.join().unwrap();


        // println!("{}", receiver.recv().unwrap());
    }

    // Arc and Mutex
    {
        // // Create a shared counter using Arc and Mutex
        // // Arc<T> == &T
        // // Mutex<T> == mut T
        // // Arc<Mutex<T>> == &mut T
        // let counter = Arc::new(Mutex::new(Cell::new(0)));
        
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
        //         let ptr = counter_reference.lock().unwrap().as_ptr();
        //         println!("Thread {} incremented counter to {:?}", i, counter_reference);
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
