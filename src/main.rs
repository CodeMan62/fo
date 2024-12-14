use clap::{Arg, Command};

fn main() {
    let matches = Command::new("File Organizer")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Organizes files in a directory")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("DIR")
                .help("Directory path to organize")
                .num_args(1) // This line ensures the argument expects a value
                .required(false),  // Makes the argument optional
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Preview changes without organizing")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        println!("Organizing directory: {}", path);
    }

    if matches.get_flag("dry-run") {
        println!("This is a dry run. No changes will be made.");
    }
}

