use config::Config;
#[cfg(target_os = "windows")]
use config::File;
use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
struct RGB {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug, Deserialize, Clone)]
struct Settings {
    title_row: String,
    leading_col: String,
    trailing_col: String,
    executable: String,
    directory: String,
    hidden: String,
}

fn reading_config_file() -> Result<Settings, Box<dyn std::error::Error>> {
    let builder = Config::builder();
    #[cfg(target_os = "windows")]
    let read_settings: Settings = builder.add_source(File::with_name("\\Applications\\btrls.toml")).build()?.try_deserialize()?;
    #[cfg(target_os = "linux")]
    let read_settings: Settings = builder.add_source(File::with_name("~/.config/btrls.toml")).build()?.try_deserialize()?;
    Ok(read_settings)
}

fn str_to_color(hex: &str) -> Result<RGB, std::num::ParseIntError> {
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(RGB { red: r, blue: b, green: g })
}