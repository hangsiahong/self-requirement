use std::collections::btree_map::BTreeMap;

struct Person {
    blood_alcohol: f32,
}

fn main() {
    // All the orders made to the bar, by client ID
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    // Our Client
    let mut blood_alcohol = BTreeMap::new();

    for id in orders {
        let person = blood_alcohol
            .entry(id)
            .or_insert(Person { blood_alcohol: 0.0 });
        person.blood_alcohol *= 0.9;

        if person.blood_alcohol > 0.3 {
            println!("Sorry {id}, I have to cut you off");
        } else {
            person.blood_alcohol += 0.1;
        }
    }
}
