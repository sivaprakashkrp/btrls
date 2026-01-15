# `btrls`
A tabled `ls` command developed in Rust with color-coded columns. Some of the basic functionality of the `ls` command has been implemented in the release `v1.2.0` with a touch to the aesthetics of the output. 

With `btrls` you can see the files and directories in a tabled format with the option to export the details of files and directories as **JSON** with the `--json` or `-j` flag.

![A sample output image of `btrls` command](./images/btrls-sample.png)

## Building the application
Clone the Repository. Make sure that you have `rustup` and `cargo` installed on you system. Then run the following command:
```bash
cargo build -r
```
Then you can find the application in the `<Path-to-repo>/target/release/btrls` or `<Path-to-repo>\target\release\btrls.exe` (If you are on a Windows Machine).

## Installation
Please refer to the releases section for the below mentioned files.
### For Windows Systems
If you are on a Windows Machine, you can download the `btrls-v1.2.0.exe` executable file and add the location of the downloaded application to the System or User Environment Variables.

And you can also compile the application from source by either downloading an archive or cloning the repository then building the application with `cargo`.

### For Unix/Linux Systems
If you are on a Linux or Mac Machine, you can download any of the archives from the releases or clone the repository and run the `install.sh` script.

**Note**: Make sure that your system already has `rustc` and `cargo` installed. 

For `bash` shell, the install script will add `btrls` to `PATH` environment variable through `.bashrc` file. If you are using any other shell, you will have to do that manually.

## Usage:
```bash
btrls [OPTIONS] [PATH]
```

## Arguments:
```bash
[PATH="."]
```
## Options:
### JSON data
```bash
-j, --json
```
Presents the current directory in JSON format

### View all files (including hidden)
```bash
-a, --all
```
Displays all the files and directories (including hidden ones)

### View Hidden Files only
```bash
-o, --only-hidden
```
Displays the hidden files and directories only

### List files and directories recursively
```bash
-r, --recursive
```
Displays the sub-directories and files recursively.

### List all files and directories (including hidden) recursively
```bash
-q, --recursive-hidden
```
Displays the all sub-directories and files (including hidden ones) recursively

### Setting Depth for Recursive Listings
```bash
-d <VALUE>, --depth <VALUE>
```
Sets the recursion depth for the recursive listings. The default value of depth is 1.

### List the attributes of a single file
```bash
-f, --file-info
```
Provides information about a single file

### View Size of Directories (Recursive)
```bash
-s, --directory-size
```
Recursively calculates the size of directories (May take time)

### Size of Directories in Bytes
```bash
-b, --byte-size
```
Displays File and Directory sizes in Bytes

### Help
```bash
-h, --help
```
Print help (see a summary with `-h`, detailed help with `--help`)

### Version
```bash
-V, --version
```  
Prints version of btrls

## Configuration

The colors in `btrls` can be configured from the `btrls-v1.2.0` build. You can add a `btrls.toml` file at `\Applications\btrls.toml` if you are in **Windows** or at `~/.config/btrls.toml` if you are in **Linux** System.

A sample of the configuration file `btrls.toml` is given here for you reference:

```toml
title_row = "#fda434"
leading_col = "#239324"
trailing_col = "#24acdd"
executable = "#425462"
directory = "#232451"
hidden = "#002432"
```

Each key takes a color code (hex values). Enclose the Hex color code in string quotes. Don't forget to add `#` in the color codes.

**Note:** This feature has been tested in **Windows** but is yet to be tested on **Linux** Platform and will be soon be tested on it.

---

*This project was started as a follow along to the video [Code Your Own CLI With Rust - Jonkero](https://youtu.be/5UA9UWWAagc?si=XNrHb2mqfUbid-dE).*