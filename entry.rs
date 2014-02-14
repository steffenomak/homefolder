use std::path::Path;
use std::io::fs::{symlink, unlink, rmdir_recursive};
use std::io::stdio::{stdin, flush, println};
use std::io::BufferedReader;
use std::os;

use utils::check_if_exists;
use utils;

pub struct Entry {
    file_name: ~str,
    path: Path,
    dir: bool
}

impl Entry {
    pub fn new(file_name: ~str, 
               path: Path,
               dir: bool) -> Entry {
        Entry{
            file_name: file_name,
            path: path,
            dir: dir
        }
    }

    pub fn new_from_array(arr: &[~str]) -> Option<Entry> {
        if arr.len() != 4 {
            return None
        }
        let mut p = Entry::check_path(arr[2].clone());
        let b = arr[3] == ~"true";
        p.push(arr[1].clone());

        Some(Entry::new(arr[0].clone(), p, b))
    }

    fn check_path(path: ~str) -> Path {
        let mut tmp: ~[&str] = path.split_str("/").collect();

        let mut i = 0;
        let home_path = match os::homedir() {
            Some(p) => p,
            None => Path::new("/root"),
        };

        let home_path = home_path.as_str().unwrap_or("fail");

        while i < tmp.len() {
            if tmp[i] == "$HOME_FOLDER$" {
                tmp[i] = home_path;
            }
            i += 1;
        }

        let mut p = Path::new("");
        p.push_many(tmp);
        p
    }

    pub fn link(&self, loc: &Path) {
        match check_if_exists(&self.path) {
            Some(f) => {
                let mut buff_read = BufferedReader::new(stdin());

                print!("{:s} exists. Remove? [Y/n]: ", 
                       self.path.as_str().unwrap());
                flush();
                let s = match buff_read.read_line() {
                    Ok(s) => s,
                    Err(_) => ~"n",
                };

                let s = s.as_slice();

                if s == "y\n" || s == "\n" || s == "Y\n" {
                    let res = match f {
                        utils::Directory => rmdir_recursive(&self.path),
                        _ => unlink(&self.path),
                    };

                    if res.is_err() {
                        println(res.to_str());
                    }

                    let res = symlink(loc, &self.path);
                    if res.is_err() {
                        println(res.to_str());
                    }
                }
            },
            None => 
            {
                let res = symlink(loc, &self.path);
                if res.is_err() {
                    println(res.to_str());
                }
            }
        };
    }
}
