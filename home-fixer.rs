use std::io::fs::{lstat, symlink, unlink, rmdir_recursive, mkdir_recursive};
use std::io;
use std::io::{BufferedReader, FilePermission, File, };
use std::io::stdio::{stdin, flush, print, println};
use std::os;

#[deriving(PartialEq)]
enum FileStatus {
    Dir,
    File,
    Symlink,
    Unknown,
    Error,
    NoneFound,
}

#[deriving(Clone)]
struct Node {
    node_location: Path,
    link_location: Path,
}

impl Node {
    pub fn new_from_path(node_location: Path, link_location: Path) -> Node {
        Node {
            node_location: node_location,
            link_location: link_location,
        }
    }

    fn exists(&self) -> FileStatus {
        match lstat(&self.link_location) {
            Ok(st) => {
                match st.kind {
                    io::TypeFile => File,
                    io::TypeDirectory => Dir,
                    io::TypeSymlink => Symlink,
                    _ => Unknown,
                }
            },
            Err(e) => {
                match e.kind {
                    io::FileNotFound => NoneFound,
                    _ => {
                        fail!("{}", e.desc);
                    },
                }
            },
        }
    }

    fn ask(message: &str) -> bool {
        let mut read = BufferedReader::new(stdin());
        print(message);
        flush();

        let ans = match read.read_char() {
            Ok(c) => c,
            Err(_) => 'n',
        };

        if ans == 'y' || ans == 'Y' || ans == '\n'{
            true
        } else {
            false
        }
    }

    pub fn link(&self) -> bool {
        let st = self.exists();
        let questin = format!("{} exists, remove it [Y/n]: ", 
                              match self.link_location.filename_str() {
                                  Some(s) => s,
                                  None => fail!("WuuWt"),
                              });
        let ask = Node::ask(questin.as_slice());

        if !ask {
            return false;
        }

        if st != NoneFound && ask {
            if st == Dir {
                match rmdir_recursive(&self.link_location) {
                    Err(_) => return false,
                    _ => (),
                };
            } else {
                match unlink(&self.link_location) {
                    Err(_) => return false,
                    _ => (),
                };
            }
        }

        match symlink(&self.node_location, &self.link_location) {
            Err(e1) => {
                match e1.kind {
                    io::FileNotFound => {
                        let mut path = self.link_location.clone();
                        path.pop();
                        let pre = FilePermission::from_bits_truncate(0o755);

                        match mkdir_recursive(&path, pre) {
                            Err(e2) => {
                                print(e2.desc);
                                false
                            },
                            _ => self.link(),
                        }
                    },
                    _ => {
                        print(e1.desc);
                        false
                    },
                }
            },
            _ => true,
        }
    }
}

struct Conf {
    home: String,
    conf_home: String,
}

fn fix_path(path: &str, conf: &Conf) -> Path {
    let parts: Vec<&str> = path.split('/').collect();
    let mut p = Path::new("");

    for part in parts.iter() {
        match *part {
            "$HOME_FOLDER$" => {
                p.push(conf.home.as_slice());
            },
            "$CONF_FOLDER$" => {
                p.push(conf.conf_home.as_slice());
            },
            _ => p.push(*part),
        };
    }

    p
}

fn parse_file(file: File, conf: &Conf) -> Vec<Node> {
    let mut reader = BufferedReader::new(file);
    let mut nodes: Vec<Node> = Vec::new();

    for line in reader.lines() {
        let l = match line {
            Ok(s) => s.as_slice().trim_right_chars('\n').to_string(),
            Err(e) => {
                println(e.desc);
                break;
            },
        };

        let clean_lines: Vec<&str> = l.as_slice().split(' ').filter(|&a| {
            !a.is_whitespace()
        }).collect();

        let node_location = fix_path(*clean_lines.get(0), conf);
        let link_location = fix_path(*clean_lines.get(1), conf);

        nodes.push(Node::new_from_path(node_location,
                                       link_location));
    };

    nodes
}

fn init_conf() -> Conf {
    let mut conf_dir = os::getcwd();
    conf_dir.push("conf");

    let home_dir = os::homedir().unwrap();

    Conf{
        home: home_dir.as_str().unwrap().to_string(),
        conf_home: conf_dir.as_str().unwrap().to_string(),
    }
}

pub fn main() {
    let conf = init_conf();

    let file = File::open(&Path::new("./conf/config.conf")).unwrap();
    let nodes = parse_file(file, &conf);

    for node in nodes.iter() {
        node.link();
    }
}
