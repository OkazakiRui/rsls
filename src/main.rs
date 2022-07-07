use std::env::current_dir;
use std::fs::read_dir;

fn main() {
    let path = current_dir().unwrap();
    let files = match read_dir(path) {
        Ok(files) => files,
        Err(_) => {
            println!("error reading directory");
            return;
        }
    };
    for file in files {
        let file_name = match file.as_ref().unwrap().file_type().unwrap().is_dir() {
            true => format!(
                "{}/",
                file.as_ref()
                    .expect("error reading file")
                    .file_name()
                    .to_str()
                    .expect("to_strでエラー")
            ),
            false => file
                .expect("error reading file")
                .file_name()
                .to_str()
                .expect("to_strでエラー")
                .to_string(),
        };
        println!("{}", file_name);
    }
}
