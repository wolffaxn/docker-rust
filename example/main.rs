use std::io::Error;
use std::io::ErrorKind;

fn echo(args: &[&str]) {
    if args.len() < 1 {
        Error::new(ErrorKind::InvalidInput, "Too few arguments");
    }
    println!("{}", args.join(" "));
}

fn main() {
    let args = ["hello,", "world"];
    echo(&args);
}
