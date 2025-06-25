use rand::Rng;
use clap::Parser;
use std::{
    collections::HashMap,
    fs,
    io,
    path::PathBuf,
};
use dirs;
use rand::seq::SliceRandom;

const COLOUR_KEYS: [&str; 19] = [
    "foreground", "background", "cursor",
    "color0", "color1", "color2", "color3", "color4", "color5", "color6", "color7",
    "color8", "color9", "color10", "color11", "color12", "color13", "color14", "color15",
];

#[derive(Parser, Debug)]
#[command(
    name = "rtc",
    author = "Rod",
    version,
    about = "Rod's Terminal Colours for Kitty",
    long_about = "Rod's Terminal Colours (rtc) is a CLI tool to manage different colour functionalities. It allows you to generate random colour schemes, create backups of your current one, load previously saved ones, print current colours, and shuffle existing colours. Colours are applied to ~/.config/kitty/kitty.kitty.conf or ~/.kitty.kitty.conf.",
)]
struct Args {
    /// Generate and apply a random Kitty colour scheme
    #[arg(short = 'r', long = "random", conflicts_with_all = &["backup", "load", "colours", "shuffle"])]
    random_colors: bool,

    /// Create a backup of your current Kitty colour configuration (only the 19 prominent colours)
    #[arg(short = 'b', long = "backup", conflicts_with_all = &["random_colors", "load", "colours", "shuffle"])]
    backup: bool,

    /// Load a saved Kitty colour configuration backup
    #[arg(short = 'l', long = "load", conflicts_with_all = &["random_colors", "backup", "colours", "shuffle"])]
    load: bool,

    /// Print the currently applied 19 prominent colours from Kitty's config
    #[arg(short = 'c', long = "colours", conflicts_with_all = &["random_colors", "backup", "load", "shuffle"])]
    colours: bool,

    /// Shuffle the currently applied 19 prominent colours in Kitty's config
    #[arg(short = 's', long = "shuffle", conflicts_with_all = &["random_colors", "backup", "load", "colours"])]
    shuffle: bool,

    /// Specify a name for the backup or load operation (e.g., 'my_theme').
    /// If not provided, a default backup/load will be used.
    #[arg(short = 'n', long = "name", value_name = "NAME")]
    name: Option<String>,
}

fn generate_random_colour_hex() -> String {
    let mut bytes = [0u8; 3];
    rand::rng().fill(&mut bytes);
    bytes.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

fn find_kitty_config_path() -> Option<PathBuf> {
    if let Some(mut path) = dirs::config_dir() {
        path.push("kitty");
        path.push("kitty.conf");
        if path.exists() {
            return Some(path);
        }
    }

    if let Some(mut path) = dirs::home_dir() {
        path.push(".kitty.conf");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn get_rtc_config_dir() -> Result<PathBuf, io::Error> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find config directory"))?;
    path.push("rtc");
    fs::create_dir_all(&path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to create RTC config directory: {}", e)))?;
    Ok(path)
}

fn get_colours_backup_path(backup_name: &Option<String>) -> Result<PathBuf, io::Error> {
    let mut path = get_rtc_config_dir()?;
    let filename = if let Some(name) = backup_name {
        format!("{}.rtc_colours", name)
    } else {
        "default.rtc_colours".to_string()
    };
    path.push(filename);
    Ok(path)
}

fn extract_current_colours(config_file_path: &PathBuf) -> Result<HashMap<String, String>, io::Error> {
    let original_content = fs::read_to_string(config_file_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read kitty.conf for colour extraction: {}", e)))?;
    let mut current_colours: HashMap<String, String> = HashMap::new();

    for line in original_content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        for &key in COLOUR_KEYS.iter() {
            if trimmed_line.starts_with(key) {
                let end_of_key_idx = key.len();
                let remaining_line = &trimmed_line[end_of_key_idx..];

                let hash_pos_in_remaining = remaining_line.find('#');

                if let Some(hash_pos) = hash_pos_in_remaining {
                    let chars_between_key_and_hash = &remaining_line[..hash_pos];
                    if chars_between_key_and_hash.trim().is_empty() {
                        let hex_code = remaining_line[hash_pos + 1..].trim();
                        if hex_code.len() == 6 && hex_code.chars().all(|c| c.is_ascii_hexdigit()) {
                            current_colours.insert(key.to_string(), hex_code.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(current_colours)
}

fn update_kitty_config_with_colours(config_file_path: &PathBuf, colours_to_apply: &HashMap<String, String>) -> Result<(), io::Error> {
    let original_content = fs::read_to_string(config_file_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read kitty.conf for update: {}", e)))?;
    let mut new_content_lines = Vec::new();

    for line in original_content.lines() {
        let mut line_modified = false;
        for &key in COLOUR_KEYS.iter() {
            let trimmed_line_start = line.trim_start();
            if trimmed_line_start.starts_with(key) {
                let remaining_after_key = &trimmed_line_start[key.len()..];
                if let Some(hash_pos_in_remaining) = remaining_after_key.find('#') {
                    let chars_between = &remaining_after_key[..hash_pos_in_remaining];
                    if chars_between.trim().is_empty() {
                        let prefix = &line[..line.len() - remaining_after_key.len()];
                        new_content_lines.push(format!("{} #{}\n", prefix.trim_end(), colours_to_apply[key]));
                        line_modified = true;
                        break;
                    }
                }
            }
        }
        if !line_modified {
            new_content_lines.push(format!("{}\n", line));
        }
    }

    let final_content = new_content_lines.join("");

    println!("Writing updated colours directly to: {}", config_file_path.display());
    fs::write(config_file_path, final_content)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to write to kitty.conf: {}", e)))?;

    Ok(())
}

fn create_colours_backup(config_file_path: &PathBuf, backup_name: Option<String>) -> Result<(), io::Error> {
    if !config_file_path.exists() {
        eprintln!("Error: kitty.conf not found at {}. Cannot create colour backup.", config_file_path.display());
        return Err(io::Error::new(io::ErrorKind::NotFound, "kitty.conf not found"));
    }

    let current_colours = extract_current_colours(config_file_path)?;
    let backup_file_path = get_colours_backup_path(&backup_name)?;

    let mut backup_content = String::new();
    for &key in COLOUR_KEYS.iter() {
        if let Some(colour_hex) = current_colours.get(key) {
            backup_content.push_str(&format!("{}#{}\n", key, colour_hex));
        } else {
            eprintln!("Warning: Colour key '{}' not found in current kitty.conf for backup. Backing up with default/missing value.", key);
            backup_content.push_str(&format!("{}#000000\n", key));
        }
    }

    println!("Creating colour backup to: {}", backup_file_path.display());
    fs::write(&backup_file_path, backup_content)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to write colour backup: {}", e)))?;

    println!("Colour backup created successfully!");
    Ok(())
}

fn load_colours_from_backup(config_file_path: &PathBuf, backup_name: Option<String>) -> Result<(), io::Error> {
    let backup_file_path = get_colours_backup_path(&backup_name)?;

    if !backup_file_path.exists() {
        eprintln!("Error: Colour backup file not found at {}. Cannot load.", backup_file_path.display());
        return Err(io::Error::new(io::ErrorKind::NotFound, "Colour backup file not found"));
    }

    let backup_content = fs::read_to_string(&backup_file_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read colour backup: {}", e)))?;
    let mut colours_to_apply = HashMap::new();

    for line in backup_content.lines() {
        if let Some(hash_pos) = line.find('#') {
            let key = line[0..hash_pos].trim();
            let hex = line[hash_pos + 1..].trim();
            colours_to_apply.insert(key.to_string(), hex.to_string());
        }
    }

    println!("Loading colours from backup: {}", backup_file_path.display());
    update_kitty_config_with_colours(config_file_path, &colours_to_apply)?;

    println!("\nKitty colours loaded from backup!");
    println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");
    Ok(())
}

fn apply_random_colours_to_kitty(config_file_path: &PathBuf) -> Result<(), io::Error> {
    let mut generated_colours_map: HashMap<String, String> = HashMap::new();
    for &key in COLOUR_KEYS.iter() {
        generated_colours_map.insert(key.to_string(), generate_random_colour_hex());
    }

    println!("\nGenerated new random colours:");
    update_kitty_config_with_colours(config_file_path, &generated_colours_map)?;

    println!("\nKitty colours updated in config file!");
    println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");
    Ok(())
}

fn print_current_colours_to_terminal(config_file_path: &PathBuf) -> Result<(), io::Error> {
    println!("Extracting current colours from: {}", config_file_path.display());
    let current_colours = extract_current_colours(config_file_path)?;

    println!("\n--- Current Kitty Colours ---");
    for &key in COLOUR_KEYS.iter() {
        if let Some(colour_hex) = current_colours.get(key) {
            println!("{}: #{}", key, colour_hex);
        } else {
            println!("{}: (Not found in config, defaulting to #000000)", key);
        }
    }
    println!("-----------------------------");

    Ok(())
}

fn shuffle_current_colours(config_file_path: &PathBuf) -> Result<(), io::Error> {
    println!("Shuffling current colours...");

    let current_colours_map = extract_current_colours(config_file_path)?;

    let mut hex_values: Vec<String> = current_colours_map.values().cloned().collect();

    if hex_values.len() != COLOUR_KEYS.len() {
        eprintln!("Warning: Found {} colour values, expected {}. Shuffle will operate on available values plus defaults for missing ones.", hex_values.len(), COLOUR_KEYS.len());
    }

    let mut rng = rand::rng();
    hex_values.shuffle(&mut rng);

    let mut shuffled_colours_map: HashMap<String, String> = HashMap::new();
    for (i, &key) in COLOUR_KEYS.iter().enumerate() {
        shuffled_colours_map.insert(key.to_string(), hex_values[i].clone());
    }

    update_kitty_config_with_colours(config_file_path, &shuffled_colours_map)?;

    println!("\nKitty colours shuffled and updated in config file!");
    println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");

    Ok(())
}


fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let config_file_path = match find_kitty_config_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: kitty.conf not found. Please ensure it's in ~/.config/kitty/kitty.conf or ~/.kitty.kitty.conf");
            return Err(io::Error::new(io::ErrorKind::NotFound, "kitty.conf not found"));
        }
    };

    if args.random_colors {
        if args.backup || args.load || args.colours || args.shuffle {
            eprintln!("Error: Cannot use --random with --backup, --load, --colours, or --shuffle.");
            return Ok(());
        }
        apply_random_colours_to_kitty(&config_file_path)?;
    } else if args.backup {
        if args.load || args.colours || args.shuffle {
            eprintln!("Error: Cannot use --backup with --load, or --colours.");
            return Ok(());
        }
        create_colours_backup(&config_file_path, args.name)?;
    } else if args.load {
        if args.colours || args.shuffle {
            eprintln!("Error: Cannot use --load with --colours, or --shuffle.");
            return Ok(());
        }
        load_colours_from_backup(&config_file_path, args.name)?;
    } else if args.colours {
        if args.shuffle {
            eprintln!("Error: Cannot use --colours with --shuffle.");
            return Ok(());
        }
        print_current_colours_to_terminal(&config_file_path)?;
    } else if args.shuffle {
        shuffle_current_colours(&config_file_path)?;
    }
    else {
        println!("No operation specified.");
        println!("Use `rtc -r` to generate colours, `rtc -b` to save, or `rtc -l` to load in, `rtc -c` to print current colours, or `rtc -s` to reorder current colours.");
        println!("Add `-n <name>` to specify `backup` or `load` file name for these operations.");
    }

    Ok(())
}
