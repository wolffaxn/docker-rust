fn echo(args: &[&str]) {
    println!("{}", args.join(" "))
}

fn main() {
    let args = ["hello,", "world"];
    echo(&args)
}
