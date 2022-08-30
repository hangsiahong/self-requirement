mod unpack_option;
use unpack_option::next_birthday;

fn main() {
    dbg!(next_birthday(Some(5)));
}
