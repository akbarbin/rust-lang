// fn main() {
//     let mut s = String::from("hello");
// 
//     s.push_str(", world!");
//     println!("{}", s);
// 
//     let s1 = "hello";
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s1);
// 
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
// 
// 
//     Ownership and Functions
//     let s = String::from("hello");
// 
//     takes_ownership(s);
//     Invalid display s
//     println!("After {}", s);
// 
//     let x = 5;
// 
//     makes_copy(x);
//     println!("After {}", x);
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

// fn main() {
//     let s1 = String::from("hello world!");
//     // let s1 = "hello";
// 
//     let (s2, len) = calculate_length(s1);
//     println!("The length of {} is {}", s2, len);
// }
// 
// fn calculate_length(s: String) -> (String, usize) {
//     let len = s.len();
// 
//     (s, len)
// }

// [4.2] References and Borrowing

// fn main() {
//     let s1 = String::from("hello");
// 
//     let len = calculate_length(&s1);
// }
// 
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Mutable References

// fn main() {
//   let mut s = String::from("hello");
// 
//   change(&mut s);
// }
// 
// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// fn main () {
//     let mut s = String::from("hello");
// 
//     {
//         let r1 = &mut s;
//         println!("{}", r1);
//     }
// 
//     let r2 = &mut s;
// 
//     println!("{}", r2);
// }

// fn main() {
//     let mut s = String::from("hello");
// 
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
// 
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point
// 
//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

// Dangling References

fn main() {
    // let reference_to_nothing = dangle();
    let _reference_to_nothing = no_dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
// 
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
