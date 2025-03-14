use std::{fs, io};

use clap::{builder::Str, command, value_parser, Arg, ArgAction};

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
        .arg(
            Arg::new("target_path")
            .help("Path to list contents from.")
            .value_parser(value_parser!(String))
            .default_value("./")
        )
        .get_matches();

    let target_path = matches.get_one::<String>("target_path").unwrap();
    if matches.get_flag("all") {
        if let Err(e) = list_files(target_path, true) {
            eprintln!("Error listing files: {}", e);
        }
    } else {
        if let Err(e) = list_files(target_path, false) {
            eprintln!("Error listing files: {}", e);
        }
    }
}

fn list_files(target_path: &str, all: bool) -> io::Result<()> {
    let entries = fs::read_dir(target_path)?;
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
