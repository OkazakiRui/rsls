use std::env::current_dir;
use std::fs::{read_dir, DirEntry};
use std::io::Error;

fn convert_file_into_string(file: Result<DirEntry, Error>) -> anyhow::Result<String> {
    let file = file?;
    let file_name = match file.file_name().into_string() {
        Ok(name) => name,
        Err(_) => return Err(anyhow::anyhow!("Could not convert file name into string")),
    };
    match file.file_type()?.is_dir() {
        true => Ok(format!("{}/", file_name).to_string()),
        false => Ok(file_name.to_string()),
    }
}

fn main() {
    let path = current_dir().expect("Could not get current directory");
    let files = match read_dir(path) {
        Ok(files) => files,
        Err(_) => {
            println!("Failed to read the directory");
            return;
        }
    };
    for file in files {
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
