use clap::{Parser};
use owo_colors::OwoColorize;
use std::{
    fs::{self}, path::PathBuf
};

// Importing from local modules
mod dependencies;
mod file_size_deps;
mod config_deps;
use crate::{dependencies::{get_data, getting_file_info, print_table, recursive_listing}};


#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about = "A tabled ls command",
    long_about = "A tabled ls command developed with Rust. Also has a option to export the contents of the directory in JSON format",
    help_template = "{bin} {version}\nDeveloped By: {author}\n\n{about}\n\nUsage:\n\t{usage}\n\n{all-args}",
    author = "Sivaprakash P"
)]
struct CLI {
    path: Option<PathBuf>,
    #[arg(short, long, help = "Presents the current directory in JSON format")]
    json: bool,
    #[arg(short, long, help = "Displays all the files and directories (including hidden ones)")]
    all: bool,
    #[arg(short = 'o', long = "only-hidden", help = "Displays the hidden files and directories only")]
    hiddenonly: bool,
    #[arg(short, long, help = "Displays the sub-directories and files recursively")]
    recursive: bool,
    #[arg(short = 'q', long = "recursive-hidden", help = "Displays the all sub-directories and files (including hidden ones) recursively")]
    recursive_hidden: bool,
    #[arg(short, long, default_value_t = 1, help = "Providing recursive depth for recursive depth of files and folders")]
    depth: u32,
    #[arg(short = 'f', long = "file-info", help = "Provides information about a file")]
    file_info: bool,
    #[arg(short = 's', long = "directory-size", help = "Recursively calculates the size of directories (May take time)")]
    directory_size: bool,
    #[arg(short='b', long = "byte-size", help = "Displays File and Directory sizes in Bytes")]
    byte_size: bool,
    #[arg(short='c', long = "config", help = "Link to a custom btrls.toml config file")]
    config_file: Option<String>,
}



fn main() {
    let cli = CLI::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));
    
    #[cfg(target_os = "windows")]
    let config_file = cli.config_file.unwrap_or(String::from("D:\\Applications\\btrls.toml"));
    #[cfg(target_os = "linux")]
    let config_file = cli.config_file.unwrap_or(String::from("~/.config/btrls.toml"));
    

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let get_files = get_data(&path, cli.all, cli.hiddenonly, cli.directory_size, cli.byte_size);
                println!(
                    "{}",
                    serde_json::to_string(&get_files).unwrap_or("Cannot Parse JSON".to_string())
                )
            } else if cli.recursive || cli.recursive_hidden {
                println!("{}", path.display());
                recursive_listing(path, cli.depth, 0, String::from(""), cli.recursive_hidden);
            } else if cli.file_info {
                let data = getting_file_info(&path);
                match data {
                    Ok(res) => print_table(config_file, res),
                    Err(msg) => println!("{}", msg.red())
                }
            } else {
                let data = get_data(&path, cli.all, cli.hiddenonly, cli.directory_size, cli.byte_size);
                match data {
                    Ok(res) => print_table(config_file, res),
                    Err(msg) => println!("{}", msg.red())
                }
            }
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error Reading the Directory".red());
    }
}

