fn main() {
    let compute = |num: i32| {
        num + 1
    };

    let num = 1;
    println!("closure_annotated: {}", compute(100));

}
