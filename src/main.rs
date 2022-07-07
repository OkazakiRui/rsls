use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::fs::{read_dir, DirEntry};

fn convert_file_into_string(file: DirEntry) -> Result<String> {
    let file_name = match file.file_name().into_string() {
        Ok(name) => name,
        Err(_) => return Err(anyhow!("Could not convert file name into string")),
    };
    match file.file_type()?.is_dir() {
        true => Ok(format!("\x1b[32m{}\x1b[m/", file_name)),
        false => Ok(file_name),
    }
}

fn main() {
    let path = current_dir().expect("Could not get current directory");
    let files = read_dir(path).expect("Failed to read the directory")

    for file in files {
        let file = file.expect("Failed to read the file");

        let file_name = match convert_file_into_string(file) {
            Ok(file_name) => file_name,
            Err(error) => {
                println!("{}", error);
                return;
            }
        };
        print!("{}  ", file_name);
    }
}
