// let write auto github push with no function, just closure, and std
fn main() {
    use std::process::Command;
    use std::env;

    let args: Vec<String> = env::args().collect();
    let message = &args[1];

    let autopush = |message: &str| {
        Command::new("git")
                .arg("add")
                .arg(".")
                .spawn()
                .expect("failed to use git add");

        Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(message)
                .spawn()
                .expect("failed to commit");

        Command::new("git")
                .arg("push")
                .spawn()
                .expect("failed to push");
    };

    // let execute = autopush(message);
    // println!("{:?}", execute);
    autopush(message);


}
