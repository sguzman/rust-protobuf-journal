extern crate chrono;
extern crate quick_protobuf;

pub mod journal;

use std::env;
use journal::JournalEntry;
use std::borrow::Cow;

fn main() {
    let journal: JournalEntry = {
        let message: Cow<str> = {
            let message: Vec<String> = env::args().skip(1).collect();
            let message: String = message.join(" ");

            Cow::from(message)
        };
        let time: u32 = 3;

        JournalEntry {
            time,
            message
        }
    };

    println!("{:?}", journal);
}
