// A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes. In this chapter, we’ll compare and contrast tuples with structs to build on what you already know and demonstrate when structs are a better way to group data. We’ll demonstrate how to define and instantiate structs. We’ll discuss how to define associated functions, especially the kind of associated functions called methods, to specify behavior associated with a struct type. Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile time type checking.

// Example of User struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut student1 = User {
        email: String::from("honghangia@gmail.com"),
        username: String::from("Hangsia"),
        active: false,
        sign_in_count: 0,

    };
    println!("{:#?}", student1);
    student1.username = String::from("Naaa");
    student1.active = true;
    println!("{:#?}", student1);

    let mut email1 =  String::from("jiren@gmail.com");
    let mut username1 =  String::from("jiren");
    let student2 = create_user(email1, username1);
    println!("{:#?}", student2);


    let student3 = User {
        email: student1.email,
        username: student1.username,
        active: student1.active,
        sign_in_count: student1.sign_in_count,
    };

    println!("This is student3 data{:#?}", student3);

    let student4 = User {
        email: String::from("Student4@gmail.com"),
        ..student2

    };
    println!("This is student4 data{:#?}", student4);
}


fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,

    }

}
