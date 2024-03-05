use std::env;

fn main() {

    let mut args: Vec<String> = Vec::new();

    for arg in env::args().skip(1) {
        args.push(arg);
    }

    for arg in args {
        println!("{}", arg);
    }

}
