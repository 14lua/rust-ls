use std::{fs, io};

use clap::{command, value_parser, Arg, ArgAction};

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
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let file_type = entry.file_type()?;
        if file_name.starts_with('.') && !all {
            continue;
        }
        let prefix = if file_type.is_dir() { "d\t" } else { "\t" };
        println!("{}{}", prefix, file_name)
    }
    Ok(())
}
