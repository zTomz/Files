use std::fs;
use colored::{Colorize, ColoredString};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory to list
    #[arg(short, long, default_value_t = String::from("."))]
    path: String,

    /// How deep to list the directory
    #[arg(short, long, default_value_t = 1)]
    depth: u32,
}

fn main() {
    let args = Args::parse();

    let directory = fs::canonicalize(args.path).unwrap();
    let depth = args.depth;

   
    let directory = directory.display().to_string().replace("\\\\?\\", "");
    println!("{}: {}", "Current directory".purple().bold(), directory);

    list_directory(directory, depth, 1);
}

fn list_directory(dir: String, max_depth: u32, depth: usize) {
    if max_depth == 0 {
        return;
    }
    let mut files: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(file) = entry {
                let path = file.path().display().to_string();
                if file.path().is_dir() {
                    println!("{}{}{}: {}", " ".repeat(depth), color_text_depth("└".to_owned(), depth), color_text_depth("Folder".to_owned(), depth), path);
                    list_directory(path, max_depth - 1, depth + 1);
                } else {
                    files.push(path);
                }
            }
        }
    }

    for file in files {
        println!("{}{}{}: {}", " ".repeat(depth), color_text_depth("└".to_owned(), depth), color_text_depth("File".to_owned(), depth), file);
    }
}

fn color_text_depth(text: String, depth: usize) -> ColoredString {
    match depth {
        1 => text.bright_blue(),
        2 => text.bright_magenta(),
        3 => text.bright_green(),
        4 => text.bright_cyan(),
        5 => text.bright_yellow(),
        6 => text.bright_purple(),
        7 => text.bright_red(),
        8 => text.blue(),
        11 => text.magenta(),
        12 => text.green(),
        13 => text.cyan(),
        14 => text.purple(),
        15 => text.red(),
        16 => text.yellow(),
        _ => text.white(),
    }
}