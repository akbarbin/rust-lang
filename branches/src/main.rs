// fn main() {
//     let number = 8;
// 
//     if number < 7 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
//

// fn main() {
//     let number = 3;
// 
//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }
//

// fn main() {
//     let mut counter = 0;
// 
//     let result = loop {
//         counter += 1;
// 
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
// 
//     println!("The result is {}", result);
// }
//

// fn main() {
//     let mut number = 3;
// 
//     while number != 0 {
//       println!("{}!", number);
// 
//       number -= 1
//     }
// 
//     println!("LISTOFF!!!");
// }


// Looping Through a Collection with 'while' 

// fn main() {
//   let a = [1, 2, 3, 4, 5];
// 
//   let mut index = 0;
// 
//   while index < 5 {
//       println!("the value is: {}", a[index]);
// 
//       index += 1;
//   }
// }

// Looping Through a Collection with 'for'

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
