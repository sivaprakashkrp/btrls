use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs::{self},
    path::{Path, PathBuf}, vec::IntoIter,
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Modify, Style, object::{Columns, Rows}
    },
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Size"}]
    len_bytes: u64,
    #[tabled{rename="Modified_At"}]
    modified: String,
    #[tabled{rename="Read_Only"}]
    read_only: bool,
    #[tabled(skip)]
    hidden: bool
}

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
}

fn main() {
    let cli = CLI::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let get_files = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&get_files).unwrap_or("Cannot Parse JSON".to_string())
                )
            } else {
                print_table(&path, cli.all, cli.hiddenonly);
            }
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error Reading the Directory".red());
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut dir_index: usize = 0;
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data, &mut dir_index);
            }
        }
    }
    data
}

fn print_table(path: &Path, all:bool, hiddenonly: bool) {
    let mut get_files = get_files(&path);
    if hiddenonly {
        let get_files_iter: IntoIter<FileEntry> = get_files.into_iter();
        get_files = only_hidden(get_files_iter);
    } else if !all {
        let get_files_iter: IntoIter<FileEntry> = get_files.into_iter();
        get_files = leave_hidden(get_files_iter);
    }
    if get_files.len() == 0 {
        println!("No Files or Directories found!");
        return;
    }
    let mut table = Table::new(&get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::last(), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_MAGENTA);
    for (i, entry) in get_files.iter().enumerate() {
        if entry.hidden {
            table.with(Modify::new(Rows::new(i+1..i+2)).with(Color::rgb_fg(128, 128, 128)));
        }
    }
    println!("{}", table);
}

fn map_dir_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize) -> fs::DirEntry {
    if let Ok(meta) = fs::metadata(&file.path()) {
        if meta.is_dir() {
            data.insert(*dir_index, FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::Dir,
                len_bytes: meta.len(),
                modified: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Utc> = mod_time.into();
                    format!("{}", date.format("%a %b %e %Y"))
                } else {
                    String::default()
                },
                read_only: meta.permissions().readonly(),
                hidden: file.file_name().into_string().unwrap().starts_with(".")
            });
            *dir_index += 1;
        }
    }
    file
}

fn map_file_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        if !meta.is_dir() {
            data.push(FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::File,
                len_bytes: meta.len(),
                modified: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Utc> = mod_time.into();
                    format!("{}", date.format("%a %b %e %Y"))
                } else {
                    String::default()
                },
                read_only: meta.permissions().readonly(),
                hidden: file.file_name().into_string().unwrap().starts_with(".")
            });
        }
    }
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize) {
    let re_arg = map_dir_data(file, data, dir_index);
    map_file_data(re_arg, data);
}

fn leave_hidden<I>(data: I) -> Vec<FileEntry> where I: Iterator<Item = FileEntry> {
    let res: Vec<FileEntry> = data.filter(|x| !x.name.starts_with(".")).collect();
    return res;
}

fn only_hidden<I>(data: I) -> Vec<FileEntry> where I: Iterator<Item = FileEntry> {
    let res: Vec<FileEntry> = data.filter(|x| x.name.starts_with(".")).collect();
    return res;
}