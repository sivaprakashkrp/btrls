use chrono::{DateTime, Local};
use std::cmp;
use clap::{Parser, builder::OsStr};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs::{self}, path::{Path, PathBuf}, vec::IntoIter
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
    len_bytes: String,
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
    #[arg(short, long, help = "Displays the sub-directories and files recursively")]
    recursive: bool,
    #[arg(short = 'q', long = "recursive-hidden", help = "Displays the all sub-directories and files (including hidden ones) recursively")]
    recursive_hidden: bool,
    #[arg(short, long, default_value_t = 1, help = "Providing recursive depth for recursive depth of files and folders")]
    depth: u32,
    #[arg(short = 'f', long = "file-info", help = "Provides information about a file")]
    file_info: bool,
}

// Files that start with "." but are not considered hidden
const SPECIAL_FILES: [&str; 1] = [".gitignore"];

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
            } else if cli.recursive || cli.recursive_hidden {
                println!("{}", path.display());
                recursive_listing(path, cli.depth, 0, String::from(""), cli.recursive_hidden);
            } else if cli.file_info {
                let data = getting_file_info(&path);
                match data {
                    Ok(res) => print_table(res),
                    Err(msg) => println!("{msg}")
                }
            } else {
                let data = get_data(&path, cli.all, cli.hiddenonly);
                match data {
                    Ok(res) => print_table(res),
                    Err(msg) => println!("{msg}")
                }
            }
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error Reading the Directory".red());
    }
}

// To convert the length of the files from Byte information to respective file length unit
pub fn convert(num: f64) -> String {
  let negative = if num.is_sign_positive() { "" } else { "-" };
  let num = num.abs();
  let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  if num < 1_f64 {
      return format!("{}{} {}", negative, num, "B");
  }
  let delimiter = 1024_f64;
  let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
  let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
  let unit = units[exponent as usize];
  format!("{}{} {}", negative, pretty_bytes, unit)
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

// To get the data about the files and directories in the given path
fn get_data(path: &Path, all:bool, hiddenonly: bool) -> Result<Vec<FileEntry>, String> {
    let mut get_files = get_files(&path);
    if hiddenonly {
        let get_files_iter: IntoIter<FileEntry> = get_files.into_iter();
        get_files = only_hidden(get_files_iter);
    } else if !all {
        let get_files_iter: IntoIter<FileEntry> = get_files.into_iter();
        get_files = leave_hidden(get_files_iter);
    }
    if get_files.len() == 0 {
        return Err(String::from("No Files or Directories found!"));
    }
    return Ok(get_files);
}

// To print the table to display
fn print_table(get_files: Vec<FileEntry>) {
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

// To collect data about directories and map them into a vector so that they can be displayed in table
fn map_dir_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize) -> fs::DirEntry {
    if let Ok(meta) = fs::metadata(&file.path()) {
        if meta.is_dir() {
            data.insert(*dir_index, FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::Dir,
                len_bytes: convert(meta.len() as f64),
                modified: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Local> = mod_time.into();
                    format!("{}", date.format("%b %e %Y %H:%M"))
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

// To collect data about files and map them into a vector so that they can be displayed in table
fn map_file_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        if !meta.is_dir() {
            data.push(FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::File,
                len_bytes: convert(meta.len() as f64),
                modified: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Local> = mod_time.into();
                    format!("{}", date.format("%b %e %Y %H:%M"))
                } else {
                    String::default()
                },
                read_only: meta.permissions().readonly(),
                hidden: file.file_name().into_string().unwrap().starts_with(".")
            });
        }
    }
}

// Calling map_dir_data and map_file_data
// This order of calling the methods is what displays Directories first, then the files in the table
fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize) {
    let re_arg = map_dir_data(file, data, dir_index);
    map_file_data(re_arg, data);
}

// To omit hidden files from the Vector
fn leave_hidden<I>(data: I) -> Vec<FileEntry> where I: Iterator<Item = FileEntry> {
    let res: Vec<FileEntry> = data.filter(|x| !x.hidden || SPECIAL_FILES.contains(&x.name.as_str())).collect();
    return res;
}

// To have only the hidden files in the Vector
fn only_hidden<I>(data: I) -> Vec<FileEntry> where I: Iterator<Item = FileEntry> {
    let res: Vec<FileEntry> = data.filter(|x| x.hidden).collect();
    return res;
}

// A function to print the structure of the data recursively
fn recursive_listing(path: PathBuf, depth:u32, count: u32, head: String, show_hidden: bool) {
    if let Ok(read_dir) = fs::read_dir(&path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if !show_hidden && file.file_name().into_string().unwrap_or("default".into()).starts_with(".") {
                    continue;
                }
                if let Ok(meta) = fs::metadata(file.path()) {
                    println!("{}├──> {}", head, file.file_name().into_string().unwrap_or("Cannot unwrap filename".into()));
                    if meta.is_dir() {
                        if count < depth {
                            recursive_listing(file.path(), depth, count + 1, format!("{}│    ", head), show_hidden);
                        }
                    }
                }
            }
        }
    }
}

// To get the data of a single file
fn getting_file_info(path: &Path) -> Result<Vec<FileEntry>, String> {
    let mut res: Vec<FileEntry> = Vec::default();
    if let Ok(meta) = fs::metadata(path) {
        res.push(FileEntry {
            name: path.file_name().unwrap_or(&OsStr::from("File Not Found!")).to_owned().into_string().expect("File Not Found!"),
            e_type: EntryType::File,
            len_bytes: convert(meta.len() as f64),
            modified: if let Ok(mod_time) = meta.modified() {
                let date: DateTime<Local> = mod_time.into();
                format!("{}", date.format("%b %e %Y %H:%M"))
            } else {
                String::default()
            },
            read_only: meta.permissions().readonly(),
            hidden: path.file_name().unwrap_or(&OsStr::from("File Not Found!")).to_owned().into_string().expect("File Not Found!").starts_with(".")
        })
    }
    if res[0].name == "File Not Found!" {
        return Err(String::from("No such file found"));
    } else {
        return Ok(res);
    }
}