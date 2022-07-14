//! Example 1
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

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangel is {} square pixels.",
//         rect1.area()
//     );
// }

//! Example 2
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 40,
//     };

//     if rect1.width() {
//         println!("The rectangel has a nonzero width; it is {}", rect1.width);
//     }
// }

//! Example 3
//#[derive(Debug)]
//struct Rectangle {
//    width: u32,
//    height: u32,
//}

//impl Rectangle {
//    fn area(&self) -> u32 {
//        self.width * self.height
//    }
//    fn can_hold(&self, other: &Rectangle) -> bool {
//        self.width > other.width && self.height > other.height
//    }
//    fn width(&self) -> bool {
//        self.width > 0
//    }
//}

//fn main() {
//    let rect1 = Rectangle {
//        width: 30,
//        height: 50,
//    };
//    let rect2 = Rectangle {
//        width: 10,
//        height: 40,
//    };
//    let rect3 = Rectangle {
//        width: 10,
//        height: 50,
//    };
//    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//}

//! Example 4

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width & self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
