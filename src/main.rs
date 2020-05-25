extern crate quick_protobuf;
extern crate time;

pub mod journal;

use journal::JournalEntry;
use quick_protobuf::Writer;
use std::env;
use std::borrow::Cow;
use std::str;
use std::str::from_utf8;
use std::time::SystemTime;
use std::io::{stdout, Write};

fn main() {
    let journal: JournalEntry = {
        let message: Cow<str> = {
            let message: Vec<String> = env::args().skip(1).collect();
            let message: String = message.join(" ");

            Cow::from(message)
        };
        let time: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get epoch").as_secs();

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

    stdout().write_all(out.as_ref());
}
