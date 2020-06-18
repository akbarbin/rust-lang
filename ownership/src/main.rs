// fn main() {
//     // let mut s = String::from("hello");
// 
//     // s.push_str(", world!");
//     // println!("{}", s);
// 
//     // let s1 = "hello";
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("{}, world!", s1);
// 
//     // let s2 = s1.clone();
//     // println!("s1 = {}, s2 = {}", s1, s2);
// 
// 
//     // Ownership and Functions
//     // let s = String::from("hello");
// 
//     // takes_ownership(s);
//     // Invalid display s
//     // println!("After {}", s);
// 
//     // let x = 5;
// 
//     // makes_copy(x);
//     // println!("After {}", x);
// }
// 
// 
// fn takes_ownership(some_string: String) {
//     println!("In {}", some_string);
// }
// 
// fn makes_copy(some_integer: i32) {
//     println!("In {}", some_integer);
// }

// Return Values and Scope

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1
// 
//     println!("s1 {}", s1);
// 
//     let s2 = String::from("hello");     // s2 comes into scope
//     println!("s2 {}", s2);
// 
//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
//     println!("s3 {}", s3);
// 
// } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
//   // moved, so nothing happens. s1 goes out of scope and is dropped.
// 
// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it
// 
//     let some_string = String::from("hello"); // some_string comes into scope
// 
//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }
// 
// // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope
// 
//     a_string  // a_string is returned and moves out to the calling function
// }
//

fn main() {
    let s1 = String::from("hello world!");
    // let s1 = "hello";

    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
} 
