extern crate quick_protobuf;

pub mod journal;

use journal::JournalEntry;
use quick_protobuf::Writer;
use std::env;
use std::borrow::Cow;
use std::str;
use std::io::{stdout, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Not enough args");
        std::process::exit(0);
    }

    let journal: JournalEntry = {
        let time: &String = &args[0];
        let time: u64 = time.parse().unwrap();

        let message: Cow<str> = {
            let message: &String = &args[1];
            Cow::from(message)
        };

        JournalEntry {
            time,
            message
        }
    };

    let mut out = Vec::new();
    {
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&journal)
            .expect("Cannot write message!");
    }

    stdout().write_all(out.as_ref())
        .expect("Could not write byte array to output");
}
