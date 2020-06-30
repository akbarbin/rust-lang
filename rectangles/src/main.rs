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

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!("rect1 is {:?}", rect1);
//
//     println!(
//         "The area of rectangles is {} square pixels.",
//         area(&rect1)
//     );
//
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of rectangles is {} square pixels.",
//         rect1.area()
//     );
//
// }

// Methods with More Parameters
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
      width: 30,
      height: 50,
    };
    let rect2 = Rectangle {
      width: 10,
      height: 40,
    };
    let rect3 = Rectangle {
      width: 60,
      height: 45,
    };
    let sq = Rectangle::square(3);

    println!(
        "The area of rectangles is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
}

// Associated Functions

