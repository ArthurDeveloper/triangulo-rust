use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct FileReader {
    pub file_content: String
}

impl FileReader {
    pub fn new(path: &str) -> FileReader {
        let mut file = File::open(path).expect("Errow while trying to open the file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Error while trying to read the file");

        FileReader {
            file_content: content
        }
    }
}
