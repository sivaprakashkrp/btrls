use config::Config;
use config::File;
use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

// SettingsFile is the struct used to read data from the configuration file
#[derive(Debug, Deserialize, Clone)]
struct SettingsFile {
    title_row: Option<String>,
    leading_col: Option<String>,
    trailing_col: Option<String>,
    executable: Option<String>,
    directory: Option<String>,
    hidden: Option<String>,
}

impl SettingsFile {
    fn to_color_config(self) -> ColorConfig {
        ColorConfig {
            title_row: str_to_hex(self.title_row),
            leading_col: str_to_hex(self.leading_col),
            trailing_col: str_to_hex(self.trailing_col),
            executable: str_to_hex(self.executable),
            directory: str_to_hex(self.directory),
            hidden: str_to_hex(self.hidden)
        }
    }
}

pub struct ColorConfig {
    pub title_row: RGB,
    pub leading_col: RGB,
    pub trailing_col: RGB,
    pub executable: RGB,
    pub directory: RGB,
    pub hidden: RGB,
}

fn reading_config_file(config_file: String) -> Result<SettingsFile, Box<dyn std::error::Error>> {
    let builder = Config::builder();
    let read_settings: SettingsFile = builder.add_source(File::with_name(&config_file)).build()?.try_deserialize()?;
    Ok(read_settings)
}

// Handling all error at once and unwrapping the RGB color value
fn str_to_hex(input: Option<String>) -> RGB {
    let hex = input.unwrap_or(String::from("#101010"));
    str_to_hex_converter(&hex).unwrap_or(RGB{ red: 10, blue: 10, green: 10})
}

// Wrapping all the error producing code in a single container function
fn str_to_hex_converter(hex: &str) -> Result<RGB, std::num::ParseIntError> {
    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;

    Ok(RGB { red: r, blue: b, green: g })
}

// Creates the ColorConfig which can by used to create colors for text in btrls
pub fn reading_config(config_file: String) -> ColorConfig {
    if let Ok(file_config) = reading_config_file(config_file) {
        // println!("Configuration loaded");
        file_config.to_color_config()
    } else {
        // println!("Failed to load Configuration");
        ColorConfig{
            title_row: RGB{red: 255, green: 0, blue: 255},
            leading_col: RGB{red: 0, green: 255, blue: 255},
            trailing_col: RGB{red: 255, green: 255, blue: 0},
            executable: RGB{red: 10, green: 255, blue: 10},
            directory: RGB{red: 10, green: 10, blue: 255},
            hidden: RGB{red: 128, green: 128, blue: 128},
        }
    }
    
}