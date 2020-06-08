fn main() {
    // 3.1 Variables

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _guess: u32 = "42".parse().expect("Not number!");

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    // 3.2 Data Types

    // Boolean
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // Character
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Tuple
    let tup = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let one = x.2;

    println!("The value of one is: {}", one);

    // Array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    let invalid = a[10];

    println!("The values: {}, {}, {}", first, second, invalid);

    // 3.3 Functions
}
