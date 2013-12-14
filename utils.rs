use std::io;
use std::path::Path;
use std::io::fs::lstat;

pub enum Type {
    File,
    Directory,
    Symlink,
    Other,
}

pub fn check_if_exists(p: &Path) -> Option<Type> {
    match io::result(|| lstat(p)) {
        Ok(s) => {
            match(s.kind) {
                io::TypeFile => Some(File),
                io::TypeDirectory => Some(Directory),
                io::TypeSymlink => Some(Symlink),
                _ => Some(Other),
            }
        },
        Err(_) => None,
    }
}

