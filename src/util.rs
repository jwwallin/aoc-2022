use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buf = String::new();
    if let Err(why) = file.read_to_string(&mut buf) {
        panic!("couldn't read {}: {}", display, why)
    }
    buf
}