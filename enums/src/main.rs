// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
// 
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
// 
//     let _home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
// 
//     let _loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// 
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }
// 
//     let _home = IpAddr::V4(String::from("127.0.0.1"));
//     let _loopback = IpAddr::V6(String::from("::1"));
// 
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String)
//     }
// 
//     let _home = IpAddr::V4(127, 0, 0, 1);
//     let _loopback = IpAddr::V6(String::from("::1"));
// }

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
// 
// fn main() {}
// 
// 
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// 
// fn main() {}
//

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
} 
