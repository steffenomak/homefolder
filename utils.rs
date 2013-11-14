use std::io::io_error;
use std::path::Path;
use std::io::fs::lstat;

pub fn check_if_exists(p: &Path) -> bool {
    let mut b = true;
    do io_error::cond.trap(|_|{b = false;}).inside {
        lstat(p);
    }

    b
}

pub fn print_path(p: &Path) {
    println(p.as_str().unwrap_or("Fail"));
}
