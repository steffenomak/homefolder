use std::io::{File, Open, ReadWrite};
use std::io::BufferedReader;
use std::os;

use entry::Entry;

mod utils;
mod entry;

fn parse_file(file: File) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();

    let mut buff_read = BufferedReader::new(file);

    for l in buff_read.lines() {
        let l = match l {
            Ok(line) => line,
            Err(_) => break,
        };

        let arr: ~[&str] = l.split_str(" ").collect();

        if arr.len() != 0 {
            match Entry::new_from_array(arr) {
                Some(e) => entries.push(e),
                None => println!("Error, amount of elements wrong: {}", 
                                 arr.len()),
            }
        }
    }

    entries
}

fn main() {
    let mut work_path = os::getcwd();

    work_path.push("conf/config.conf");

    match utils::check_if_exists(&work_path) {
        None => fail!("Config file missing"),
        _ => (),
    }

    let file = File::open_mode(&work_path, Open, ReadWrite).unwrap();

    work_path.pop();
    let entries = parse_file(file);

    for e in entries.iter() {
        work_path.push(e.file_name.clone());
        e.link(&work_path);
        work_path.pop();
    }
}

