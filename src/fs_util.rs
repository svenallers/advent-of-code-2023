use std::fs::read_to_string;
use std::path::Path;

pub fn read_or_panic(path: impl AsRef<Path>) -> String {
    let Ok(data) = read_to_string(path) else {
        panic!("unable to read file")
    };
    return data;
}