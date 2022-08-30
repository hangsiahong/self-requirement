use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    name: String,
    origin: String,
}

impl Person {
    fn new(name: &str, origin: &str) -> Self {
        Self {
            name: name.to_string(),
            origin: origin.to_string(),
        }
    }
}

fn main() {
    let person1 = HashMap::from([
        (Person::new("Jiren", "Alien"), 25),
        (Person::new("Goku", "Saiyajin"), 25),

    ]);

    for (person, health) in &person1 {
        println!("{person:?} has {health} hp");

    }


}
