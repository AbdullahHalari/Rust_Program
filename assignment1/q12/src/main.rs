

use std::io;

fn main() {
    println!("what do you want to write?");

    let mut to_print = String::new();

    io::stdin().read_line(&mut to_print)
    .expect("failed to read line");

    println!("{}", to_print);

}