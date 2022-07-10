fn main() {
    another_function();
    another_function_with_parameter(100);
    multiple_parameter(10, 'T');
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
