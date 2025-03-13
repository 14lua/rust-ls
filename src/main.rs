use std::{fs, io, path::{self, Path}};

use clap::{command, Arg, ArgAction, Command};

fn main() {
    let matches = command!()
        .about("Lua's recreation of the 'ls' coreutil command.")
        .arg(
            Arg::new("all")
            .help("Additionally show hidden files.")
            .short('a')
            .long("all")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    if matches.get_flag("all") {
        if let Err(e) = list_files(true) {
            eprintln!("Error listing files: {}", e);
        }
    } else {
        if let Err(e) = list_files(false) {
            eprintln!("Error listing files: {}", e);
        }
    }
}

fn list_files(all: bool) -> io::Result<()> {
    match fs::read_dir("./") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if let Some(first_char) = file_name.chars().nth(0) {
                            if first_char == '.' && !all {
                                continue;
                            }
                        }
                        println!("{}", file_name);
                    } else {
                        println!("Invalid UTF-8");
                    }
                }
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    Ok(())
}
