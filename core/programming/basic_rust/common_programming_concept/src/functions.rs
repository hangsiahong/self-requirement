fn main() {
    another_function();
    another_function_with_parameter(100);
    multiple_parameter(10, 'T');
    println!("{}",return_value(100));
    println!("{}",plus_one(50));
}

fn another_function() {
    println!("This is another function");

}

fn another_function_with_parameter(x: i32){
    println!("The value of the parameter is: {x}");

}

fn multiple_parameter(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");

}

fn return_value(value: i32) -> i32 {
    value

}

fn plus_one(x: i32) -> i32 {
    x + 1

}
