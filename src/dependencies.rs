use chrono::{DateTime, Local};
use clap::{builder::OsStr};
use is_executable::IsExecutable;
use serde::Serialize;
use std::{
    fs::{self}, path::{Path, PathBuf}, vec::IntoIter
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Modify, Style, object::{Cell, Columns, Rows}
    },
};
use hf::is_hidden;

// Importing from file_size_deps.rs
use crate::file_size_deps::{convert, find_length};

#[derive(Debug, Display, Serialize, PartialEq, Eq)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
pub struct FileEntry {
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
    hidden: bool,
    #[tabled(skip)]
    is_exec: bool,
}

// Files that start with "." but are not considered hidden
const SPECIAL_FILES: [&str; 1] = [".gitignore"];

fn get_files(path: &Path, directory_size: bool, byte_size: bool) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut dir_index: usize = 0;
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data, &mut dir_index, directory_size, byte_size);
            }
        }
    }
    data
}

// To get the data about the files and directories in the given path
pub fn get_data(path: &Path, all:bool, hiddenonly: bool, directory_size: bool, byte_size: bool) -> Result<Vec<FileEntry>, String> {
    let mut get_files = get_files(&path, directory_size, byte_size);
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
pub fn print_table(get_files: Vec<FileEntry>) {
    let mut table = Table::new(&get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::last(), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_MAGENTA);
    for (i, entry) in get_files.iter().enumerate() {
        if entry.e_type == EntryType::Dir && !entry.hidden {
            table.with(Modify::new(Cell::new(i+1, 1)).with(Color::rgb_fg(10, 10, 225)));
        }
        if entry.is_exec {
            table.with(Modify::new(Cell::new(i+1,1)).with(Color::rgb_fg(10, 225, 10)));
        }
        if entry.hidden {
            table.with(Modify::new(Rows::new(i+1..i+2)).with(Color::rgb_fg(128, 128, 128)));
        }
    }
    println!("{}", table);
}

// To collect data about directories and map them into a vector so that they can be displayed in table
fn map_dir_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize, directory_size: bool, byte_size: bool) -> fs::DirEntry {
    if let Ok(meta) = fs::metadata(&file.path()) {
        if meta.is_dir() {
            data.insert(*dir_index, FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::Dir,
                len_bytes: find_length(&file.path(), directory_size, byte_size),
                modified: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Local> = mod_time.into();
                    format!("{}", date.format("%b %e %Y %H:%M"))
                } else {
                    String::default()
                },
                read_only: meta.permissions().readonly(),
                hidden: is_hidden(&file.path()).unwrap_or(false),
                is_exec: file.path().is_executable(),
            });
            *dir_index += 1;
        }
    }
    file
}

// To collect data about files and map them into a vector so that they can be displayed in table
fn map_file_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, byte_size: bool) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        if !meta.is_dir() {
            data.push(FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::File,
                len_bytes: find_length(&file.path(), false, byte_size),
                modified: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Local> = mod_time.into();
                    format!("{}", date.format("%b %e %Y %H:%M"))
                } else {
                    String::default()
                },
                read_only: meta.permissions().readonly(),
                hidden: is_hidden(&file.path()).unwrap_or(false),
                is_exec: file.path().is_executable(),
            });
        }
    }
}

// Calling map_dir_data and map_file_data
// This order of calling the methods is what displays Directories first, then the files in the table
fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize, directory_size: bool, byte_size: bool) {
    let re_arg = map_dir_data(file, data, dir_index, directory_size, byte_size);
    map_file_data(re_arg, data, byte_size);
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
pub fn recursive_listing(path: PathBuf, depth:u32, count: u32, head: String, show_hidden: bool) {
    if let Ok(read_dir) = fs::read_dir(&path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if !show_hidden && is_hidden(&file.path()).unwrap_or(false) {
                    continue;
                }
                if let Ok(meta) = fs::metadata(file.path()) {
                    println!("{}├──> {}{}", head, if file.path().is_executable() {"*"} else {""}, file.file_name().into_string().unwrap_or("Cannot unwrap filename".into()));
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
pub fn getting_file_info(path: &Path) -> Result<Vec<FileEntry>, String> {
    let mut res: Vec<FileEntry> = Vec::default();
    if let Ok(meta) = fs::metadata(path) {
        res.push(FileEntry {
            name: path.file_name().unwrap_or(&OsStr::from("File Not Found!")).to_owned().into_string().expect("File Not Found!"),
            e_type: if meta.is_dir() {EntryType::Dir} else {EntryType::File},
            len_bytes: convert(meta.len() as f64),
            modified: if let Ok(mod_time) = meta.modified() {
                let date: DateTime<Local> = mod_time.into();
                format!("{}", date.format("%b %e %Y %H:%M"))
            } else {
                String::default()
            },
            read_only: meta.permissions().readonly(),
            hidden: is_hidden(&path).unwrap_or(false),
            is_exec: path.is_executable(),
        })
    }
    if res[0].name == "File Not Found!" {
        return Err(String::from("No such file found"));
    } else {
        return Ok(res);
    }
}