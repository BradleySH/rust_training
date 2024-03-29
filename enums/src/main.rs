// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

//  OPTION
#![allow(unused)]
fn main() {
    enum Option<T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
