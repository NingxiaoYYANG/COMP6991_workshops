#![allow(unused)]
fn main() {
    // let mut data = vec![1, 2, 3, 4, 5];
    // // let (left, right) = (&mut data[..2], &mut data[2..]);

    // // But this is actually safe - they don't overlap!
    // // Working Part !!!!

    // let (left, right) = my_vec::split_at_mut(&mut data, 2);
    // left[0] = 10;
    // right[0] = 20;
    // println!("{:?}", data); // [10, 2, 20, 4, 5]
    
    {
    let mut x = String::from("Hello");
    let a = String::from("hello2");



    let immutable_ptr: *const String = &a;
    println!("{:?}", immutable_ptr);
    let mutable_ptr: *mut String = &mut x as *mut String;
    println!("{:?}", mutable_ptr);

    // unsafe {
    //     println!("{}", *immutable_ptr);
    //     println!("{}", *mutable_ptr);
    // }
    unsafe { 
     // immutable_ptr.offset(1) 
     (*mutable_ptr).push_str(", world!");
    //  (*immutable_ptr).push_str(", world!");

     println!("{}", *mutable_ptr);
    };
    }
}











mod my_vec {
    pub struct MyVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    impl<T> MyVec<T> {
        pub fn push(&mut self, value: T) {
            // Unsafe code here, but the API is safe to use
        }
        pub fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                /// SAFETY: index is checked against len
                unsafe { Some(&*self.ptr.add(index)) }
            } else {
                None
            }
        }
    }
    /// Safely splits a mutable slice into two non-overlapping parts
    pub fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        // Validate the split point
        assert!(mid <= len, "split index out of bounds");
        
        // SAFETY:
        // - mid <= len, so both slices are within bounds
        // - The two slices don't overlap
        // - The original slice is borrowed mutably, preventing other access
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}