// use std::fs::File;
// use std::io::ErrorKind;
// 
// fn main() {
//     let f = File::open("hello.txt");
// 
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc, //                 Err(e) => panic!("Problem creating the file: {:?}", e),
//         },
//         other_error => {
//             panic("Problem opening the file: {:?}", other_error)
//         }
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;
// 
// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }
//

// use std::fs::File;
// 
// fn main() {
//     let f = File::open("hello.txt").unwrap();
// }
//

#![allow(unused_variables)]
// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;
// 
//     read_username_from_file();
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let f = File::open("hello.txt");
// 
//         let mut f = match f {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };
// 
//         let mut s = String::new();
// 
//         match f.read_to_string(&mut s) {
//             Ok(_) => Ok(s),
//             Err(e) => Err(e),
//         }
//     }
// }

// use ? operator
// #![allow(unused_variables)]
// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;
// 
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut f = File::open("hello.txt")?;
//         let mut s = String::new();
//         f.read_to_string(&mut s)?;
//         Ok(s)
//     }
// }
//

// #![allow(unused_variables)]
// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;
// 
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut f = File.open("hello.txt")?.read_to_string(&mut s)?;
// 
//         Ok(s)
//     }
// }

// #![allow(unused_variables)]
// fn main() {
//     use std::fs;
//     use std::io;
// 
//     read_username_from_file();
//     fn read_username_from_file() -> Result<String, io::Error> {
//         fs::read_to_string("hello.txt")
//     }
// }
//

// The ? Operator Can Be Used in Functions That Return Result
#![allow(unused_variables)]
// use std::fs::File;
// 
// fn main() {
//     let f = File::open("hello.txt")?;
// }

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
