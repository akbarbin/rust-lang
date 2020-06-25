// fn main() {
//     let width = 30;
//     let height = 50;
// 
//     println!(
//         "The area of rectangles is {} square pixels.",
//         area(width, height)
//     );
// }
// 
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
//

// 5.9 Specifying the width and height of the rectangle with a tuple

// fn main() {
//     let react1 = (30, 50);
//     println!(
//         "The area of rectangles is {} square pixels.",
//         area(react1)
//     );
// 
// }
// 
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Refactoring with Structs: Adding More Meaning

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of rectangles is {} square pixels.",
        area(&rect1)
    );

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

