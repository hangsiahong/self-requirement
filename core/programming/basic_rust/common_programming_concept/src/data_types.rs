// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize
// no sign = possitive
// sign = possitive + negative


// Number literals	Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

// Not sure what type to use? Rust default got you `i32`.

// Floating point type
// Rust have two primitives types for floating point numbers;
// f32 = float 32bit
// f64 = float 64bit
fn floting_types() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

}

// Numeric Operations
fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn bool_type() {
    let t = true;
    let f: bool = false;

}

fn character_type() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

// Compount types
// Compound types can group multiple value into one type. Rust has
// two primitives compound types which are Tuple and Array.

// Tuple Example
fn tuple_type() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("tup {:?}", tup);

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;
}

// Array example
fn array_type() {
    let array1 = [1, 2, 3, 4, 5];
    let contain_6: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", contain_6);
}
fn invalid_array_access() {
    use std::io;

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("failed to readline");
    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

}

fn main() {
    // array_type();
    invalid_array_access();

}
