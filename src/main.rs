use std::{fs, io};

use clap::{command, Arg, ArgAction};

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
    let entries = fs::read_dir("./")?;
    println!("\tName\n\t————");
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if let Ok(file_type) = entry.file_type() {
                    if let Some(first_char) = file_name.chars().nth(0) {
                        if first_char == '.' && !all {
                            continue;
                        }
                    }
                    let mut prefix = String::from("\t");
                    if file_type.is_dir() {
                        prefix = format!("d{prefix}");
                    }
                    println!("{}{}", prefix, file_name)
                }
            } else {
                println!("Invalid UTF-8");
            }
        }
    }
    Ok(())
}
