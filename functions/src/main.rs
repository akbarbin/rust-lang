fn main() {
    // println!("Hello, world!");

    // let x = 5;

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The value of y is: {}", y);

    // another_function();
    // fn_with_params(5, 6);

    // Functions with Return Values

    // let x = five();
    // println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// fn another_function() {
//   println!("Another function.");
// }
// 
// fn fn_with_params(x: u32, y: u32) {
//   println!("The value of x is : {}", x);
//   println!("The value of y is : {}", y);
// }

// fn five() -> i32 {
//   5
// }

fn plus_one(x: i32) -> i32 {
  x + 1
}
