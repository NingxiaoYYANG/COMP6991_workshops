use std::collections::vec_deque;

use libc::{fopen, fgets, printf, fclose, FILE, c_char, c_int, c_double};

unsafe extern "C" {
    fn scanf(stream: *mut FILE, format: *const c_char) -> c_int;
}
struct File {
    file: *mut FILE,
    size: usize,
}

/// This function converts a string into a Vec<i8> which can
/// be used to represent a c-string.
///
/// To turn this into a *mut c_char, use Vec<i8>::as_mut_ptr().
fn to_c_string(string: &str) -> Vec<i8> {
    let bytes: Vec<u8> = String::from(string).into_bytes();
    let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect();

    c_chars.push(0); // null terminator

    c_chars
}

impl File {
    fn open(_path: &str) -> Option<Self> {
        let file = unsafe {
            fopen(to_c_string(_path).as_ptr(), to_c_string("r").as_ptr())
        };
        if file.is_null() {
            None
        } else {
            Some(File {file, size: 128})
        }

    }

    fn read_string(&mut self) -> Option<String> {
        let mut buff: Vec<i8> = vec![0; self.size];

        let failed = unsafe {
            fgets(buff.as_mut_ptr(), self.size as i32, self.file).is_null()
        };

        if failed {
            None 
        } else {
            let mut chars = vec![];
            for i in buff {
                if i == 0 {
                    break;
                }
                chars.push((i as u8) as char);
            }
            Some(chars.iter().collect::<String>())
        }
    }

    fn read_i64(&mut self) -> Option<i64> {
        unsafe {
            // let mut dec: c_int = 0 as c_int;
            // let num_scanned = fscanf(self.file, to_c_string(" %d").as_ptr(), &mut dec);
            // if num_scanned == 1 {
            //     Some(dec as i64)
            // } else {
            //     None
            // }
        }
        None
    }

    fn read_f64(&mut self) -> Option<f64> {
        todo!()
    }

    fn read_char(&mut self) -> Option<char> {
        todo!()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        println!("Dropping file.");
    }
}


fn main() {
    let mut file = File::open("data/test_file.txt").expect("Could not open file.");
    // let s = file.read_string().unwrap();
    // dbg!(s);
    
    // let i = file.read_i64().unwrap();
    // let f = file.read_f64().unwrap();
    // let c = file.read_char().unwrap();

    // println!("{s} {i} {f} {c}");

    unsafe { printf(to_c_string("adhiasdhiasdhioa").as_ptr()) };
}
