struct S; // Concrete type `S`
struct GenericVal<T>(T); // Generic type

// impl of GenericVal where we explicitly specify type parameters:
impl GenericVal<f32> {} // specify `f32`
impl GenericVal<S> {} // specify `S` as defined above
 
// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// Implement of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{} {}", x.value(), y.value());
}

