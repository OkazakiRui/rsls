use anyhow::{Ok, Result};
use std::env::current_dir;
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsls", about = "LS command made by Rust")]
struct Options {
    #[structopt(short = "1", long = "--format=single-column")]
    is_single_column: bool,
    #[structopt(short = "a", long = "--all")]
    show_dotfiles: bool,
    #[structopt(name = "FILE", parse(from_os_str))]
    path: Option<PathBuf>,
}

fn convert_file_into_string(file: DirEntry) -> Result<String> {
    let file_name = file.file_name().to_string_lossy().to_string();
    match file.file_type()?.is_dir() {
        true => Ok(format!("{}/", file_name)),
        false => Ok(file_name),
    }
}

fn color_the_letters(file_name: String) -> String {
    match file_name.ends_with("/") {
        true => format!("\x1b[32m{}\x1b[m", file_name),
        false => file_name,
    }
}

fn run(options: Options) -> Result<()> {
    let path = match options.path {
        Some(path) => path,
        None => current_dir()?,
    };
    let files = read_dir(path)?;

    for file in files {
        let file_name = convert_file_into_string(file?)?;
        if !options.show_dotfiles && file_name.starts_with(".") {
            continue;
        };
        let file_name = color_the_letters(file_name);

        match options.is_single_column {
            true => println!("{}", file_name),
            false => print!("{}  ", file_name),
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Options::from_args()) {
        eprintln!("ERROR: {}", e);
    }
}
