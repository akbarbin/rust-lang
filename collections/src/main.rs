fn main() {
    // Listing 8-1: Creating a new, empty vector to hold values of type i32
    // let v: Vec<i32> = Vec::new();

    // Listing 8-2: Creating a new vector containing values
    // let v = vec![1, 2, 3];

    // Updating vector
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // Dropping a Vector Drops Its Elements
    // {
    //     let v = vec![1, 2, 3];
    // } // <- v goes out of scope and is freed here

    // Reading Elements of Vectors
    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element"),
    // }

    // Attempting to access the element at index 100 in a vector containing five elements
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // Attempting to add an element to a vector while holding a reference to an item
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is {}", first);

    // Iterating over the Values in a Vector
    // let v = vec![100, 32, 57];

    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 57];

    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }

    // Using an Enum to Store Multiple Types
    fn main() {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
