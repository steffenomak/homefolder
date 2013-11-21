use std::io;
use std::path::Path;
use std::io::fs::lstat;

pub fn check_if_exists(p: &Path) -> bool {
    match io::result(|| lstat(p)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn print_path(p: &Path) {
    println(p.as_str().unwrap_or("Fail"));
}
