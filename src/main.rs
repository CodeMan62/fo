mod config;
mod file_scaner;
mod categorizer;
mod organizer;

use clap::{Arg, Command};
use config::load_config;
use organizer::organize_files;

fn main() {

    let matches = Command::new("fo")
        .about("A file organizer CLI tool")
        .arg(Arg::new("path")
            .long("path")
            .help("Path to the directory to organize")
            .required(true)
            .value_name("PATH"))
        .arg(Arg::new("dry-run")
            .long("dry-run")
            .help("Run without moving files"))
        .arg(Arg::new("config")
            .long("config")
            .help("Path to the configuration file")
            .value_name("CONFIG"))
        .get_matches();
    let path = matches.get_one::<String>("path").expect("Path is required");
    let dry_run = matches.contains_id("dry-run");

    // Fix for config_path
    let default_config = "config.json".to_string();
    let config_path = matches.get_one::<String>("config").unwrap_or(&default_config);

    let config = load_config(config_path).expect("Failed to load configuration");
    organize_files(path, &config, dry_run).expect("Error organizing files");
}

