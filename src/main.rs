extern crate quick_protobuf;

pub mod journal;

use std::env;
use journal::JournalEntry;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args: String = args.join(" ");
    println!("{}", args);
}
