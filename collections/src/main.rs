// fn main() {
//     // Listing 8-1: Creating a new, empty vector to hold values of type i32
//     // let v: Vec<i32> = Vec::new();
//
//     // Listing 8-2: Creating a new vector containing values
//     // let v = vec![1, 2, 3];
//
//     // Updating vector
//     // let mut v = Vec::new();
//
//     // v.push(5);
//     // v.push(6);
//     // v.push(7);
//     // v.push(8);
//
//     // Dropping a Vector Drops Its Elements
//     // {
//     //     let v = vec![1, 2, 3];
//     // } // <- v goes out of scope and is freed here
//
//     // Reading Elements of Vectors
//     // let v = vec![1, 2, 3, 4, 5];
//
//     // let third: &i32 = &v[2];
//     // println!("The third element is {}", third);
//
//     // match v.get(2) {
//     //     Some(third) => println!("The third element is {}", third),
//     //     None => println!("There is no third element"),
//     // }
//
//     // Attempting to access the element at index 100 in a vector containing five elements
//     // let v = vec![1, 2, 3, 4, 5];
//     // let does_not_exist = &v[100];
//     // let does_not_exist = v.get(100);
//
//     // Attempting to add an element to a vector while holding a reference to an item
//     // let mut v = vec![1, 2, 3, 4, 5];
//
//     // let first = &v[0];
//
//     // v.push(6);
//
//     // println!("The first element is {}", first);
//
//     // Iterating over the Values in a Vector
//     // let v = vec![100, 32, 57];
//
//     // for i in &v {
//     //     println!("{}", i);
//     // }
//
//     // let mut v = vec![100, 32, 57];
//
//     // for i in &mut v {
//     //     *i += 50;
//     //     println!("{}", i);
//     // }
//
//     // Using an Enum to Store Multiple Types
//     // fn main() {
//     //     enum SpreadsheetCell {
//     //         Int(i32),
//     //         Float(f64),
//     //         Text(String),
//     //     }
//
//     //     let row = vec![
//     //         SpreadsheetCell::Int(3),
//     //         SpreadsheetCell::Text(String::from("blue")),
//     //         SpreadsheetCell::Float(10.12),
//     //     ];
//     // }
//
//     // Storing UTF-8 Encoded Text with Strings
//     let data = "initial contents";
//
//     let s = data.to_string();
//
//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
//
//     // storing greetings in different languages in strings
//     let hello = String::from("السلام عليكم");
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("שָׁלוֹם");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     let hello = String::from("Hola");
//
//     // Appending string with push_str
//     let mut s = String::from("foo");
//     s.push_str("bar");
//
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2 is {}", s2);
//
//     // push string with one character
//     let mut s = String::from("lo");
//     s.push('l');
//
//     // Concatenation with the + operator or the format! macro
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
//
//     // using format
//
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//
//     let s = format!("{}-{}-{}", s1, s2, s3);
//
//     // Indexing into Strings
//     let s1 = String::from("hello");
//     // let h = s1[0]; // note h has an integer error indexing
//
//     // Internal Representation
//     let hello = String::from("Hola");
//
//     let hello = String::from("Здравствуйте");
//
//     let hello = "Здравствуйте";
//     // let answer = &hello[0];
//
//     // Slicing Strings
//     let hello = "Здравствуйте";
//
//     let s = &hello[0..4];
//     println!("s = {}", s);
//
//     // Methods for Iterating Over Strings
//     for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }
//
//     for b in "नमस्ते".bytes() {
//         println!("{}", b);
//     }
// }

// fn main() {
//     // // Creating new Hash Map
//     // use std::collections::HashMap;
//
//     // let mut scores = HashMap::new();
//
//     // scores.insert(String::from("Blue"), 10);
//     // scores.insert(String::from("Yellow"), 50);
//
//     // // Creating a hash map from a list of teams and a list of scores
//     // let teams = vec![String::from("Blue"), String::from("Yellow")];
//     // let initial_scores = vec![10, 50];
//
//     // let mut scores: HashMap<_, _> =
//     //     teams.into_iter().zip(initial_scores.into_iter()).collect();
//
//     // // Hash Maps and Ownership
//     // let field_name = String::from("Favorite color");
//     // let field_value = String::from("Blue");
//
//     // let mut map = HashMap::new();
//     // map.insert(field_name, field_value);
//
//     // // field_name and field_value are invalid at this point, try using them and
//     // // see what compiler error you get!
//     // // println!("field_name = {} field_value = {}", field_name, field_value);
//
// }

// Accessing Hash Map
// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);
//     println!("{:?}", score);
//     println!("{}", team_name);
//
//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

// Updating Hash Map
// Overwritting a Value
// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);
//
//     println!("{:?}", scores);
// }

// Only Inserting a Value If the Key Has No Value
// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//
//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
//
//     println!("{:?}", scores);
// }

// Updating a Value Based on the Old Value
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}", count);
    }

    println!("{:?}", map);
}
