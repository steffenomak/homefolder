use std::io::{File, Open, ReadWrite};
use std::io::buffered::BufferedReader;
use std::os;

use entry::Entry;

mod utils;
mod entry;

fn parse_file(file: File) -> ~[Entry] {
    let mut entries: ~[Entry] = ~[];

    let mut buff_read = BufferedReader::new(file);

    while !buff_read.eof() {
        let s = match buff_read.read_line() {
            Some(s) => s,
            None => break,
        };

        let mut str_list: ~[~str] = ~[];

        for part in s.words() {
            str_list.push(part.into_owned());
        }

        if str_list.len() == 0 {
            break;
        }

        match Entry::new_from_array(str_list) {
            Some(e) => entries.push(e),
            None => {
                println(format!("Error, nr in str_list: {:u}", str_list.len()));

                for i in str_list.iter() {
                    println(*i);
                }
            },
        }
    }

    entries
}

fn main() {
    let mut work_path = os::getcwd();

    work_path.push("conf/config.conf");

    if !utils::check_if_exists(&work_path) {
        fail!("Config file missing.");
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

