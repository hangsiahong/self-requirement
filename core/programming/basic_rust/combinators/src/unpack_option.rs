pub fn next_birthday(current_age: Option<u8>) -> Option<String> {

    let next_age = current_age? + 1;
    Some(format!("Next Year i will be {}", next_age))
}
