// A HashSet implemented as a `HashMap` where the value is ().

use std::collections::HashSet;


fn main() {
    let mut spell_book = HashSet::new();
    // Add some spells to the book,
    spell_book.insert("Fire Ball".to_string());
    spell_book.insert("Water Ball".to_string());
    spell_book.insert("Lighting Fury".to_string());
    spell_book.insert("Wind Shield".to_string());

    // Check for a specific spell
    if !spell_book.contains("Lighting Claw") {
        println!("We have {} spells, but Lighting Class is not one of them", spell_book.len());
    }
    spell_book.remove("Fire Ball");

    for spell in spell_book {
        println!("{spell}");
    }

}
