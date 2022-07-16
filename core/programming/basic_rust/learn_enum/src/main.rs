//! Enumerations, also known as enums. Enums allow you to define a type by enumerating its possible variants. First, we'll
//! define and use an enum to show how an enum can encode meaning along with data. Next, we'll explore a useful enum, called
//! `Option`, which expresses that a value can be either something or nothing. Then we will look at how pattern mataching
//! expresion makes it easy to run different code for different values of an enum. Finally, we will earn how the `if let`
//! construct is another convenient and concise idiom available to handle enums in our code.

// Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to
// `algebraic data types` in functional languages, such as F#, OCaml, and Haskell.

//! `IpAddrKind` is  now a custom data type that we can elsewhere in our code.
// enum IpAddrKind {
//     V4,
//     V6,
// }
//! We can create instances of each of the two variants of `IpAddrKind` like this:
// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;


//! Example 1
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }


// fn main() {
//     println!("Learn Enum");
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("{:?}", home);
//     println!("{:?}", loopback);
// }

//! Example 2 Without struct
//#[derive(Debug)]
//enum IpAddrKind {
//    v4(String),
//    v6(String),

//}

//fn main() {
//    let home = IpAddrKind::v4(String::from("127.0.0.1"));
//    let loopback = IpAddrKind::v6(String::from("::1"));
//    println!("{:?}", home);
//    println!("{:?}", loopback);
//}

//! Example 3
struct Ipv4Addr {
    address: String,
}

struct Ipv4Addr {
    address: String,
}
