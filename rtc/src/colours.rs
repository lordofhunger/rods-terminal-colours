use std::{collections::HashMap, fs, io, path::PathBuf};
use crate::config::get_colours_backup_path;
use crate::util::generate_random_colour_hex;
use rand::seq::SliceRandom;

lazy_static::lazy_static! {
    pub static ref COLOUR_KEY_ALIASES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("fg", "foreground");
        m.insert("bg", "background");
        m.insert("cs", "cursor");
        m.insert("c0", "color0");
        m.insert("c1", "color1");
        m.insert("c2", "color2");
        m.insert("c3", "color3");
        m.insert("c4", "color4");
        m.insert("c5", "color5");
        m.insert("c6", "color6");
        m.insert("c7", "color7");
        m.insert("c8", "color8");
        m.insert("c9", "color9");
        m.insert("c10", "color10");
        m.insert("c11", "color11");
        m.insert("c12", "color12");
        m.insert("c13", "color13");
        m.insert("c14", "color14");
        m.insert("c15", "color15");
        m
    };
}

pub const COLOUR_KEYS: [&str; 19] = [
    "foreground", "background", "cursor",
    "color0", "color1", "color2", "color3", "color4", "color5", "color6", "color7",
    "color8", "color9", "color10", "color11", "color12", "color13", "color14", "color15",
];

pub fn extract_current_colours(config_file_path: &PathBuf) -> Result<HashMap<String, String>, io::Error> {
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

pub fn update_kitty_config_with_colours(config_file_path: &PathBuf, colours_to_apply: &HashMap<String, String>) -> Result<(), io::Error> {
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

pub fn create_colours_backup(config_file_path: &PathBuf, backup_name: Option<String>) -> Result<(), io::Error> {
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

pub fn load_colours_from_backup(config_file_path: &PathBuf, backup_name: Option<String>) -> Result<(), io::Error> {
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

    println!("Loading colours from backup: {}", config_file_path.display());
    update_kitty_config_with_colours(config_file_path, &colours_to_apply)?;

    println!("\nKitty colours loaded from backup!");
    println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");
    Ok(())
}

pub fn parse_color_keys_input(input: &Option<String>) -> Vec<String> {
    let mut result_keys = Vec::new();
    if let Some(s) = input {
        let cleaned = s.trim_start_matches('(').trim_end_matches(')').to_string();
        for part in cleaned.split(',') {
            let trimmed_part = part.trim();
            if trimmed_part.is_empty() {
                continue;
            }

            if let Some(&full_key) = COLOUR_KEY_ALIASES.get(trimmed_part) {
                result_keys.push(full_key.to_string());
            } else if COLOUR_KEYS.contains(&trimmed_part) {
                result_keys.push(trimmed_part.to_string());
            } else {
                eprintln!("Warning: Unknown colour key or alias '{}' provided in list. It will be ignored.", trimmed_part);
            }
        }
    }
    result_keys
}

pub fn apply_random_colours_to_kitty(
    config_file_path: &PathBuf,
    exception_keys_input: &Option<String>,
    force_keys_input: &Option<String>,
) -> Result<(), io::Error> {
    let current_colours = extract_current_colours(config_file_path)?;
    let mut generated_colours_map: HashMap<String, String> = HashMap::new();

    let forced_keys = parse_color_keys_input(force_keys_input);
    let excluded_keys = parse_color_keys_input(exception_keys_input);

    for &key in COLOUR_KEYS.iter() {
        let key_string = key.to_string();

        let should_randomize = if !forced_keys.is_empty() {
            forced_keys.contains(&key_string)
        } else {
            !excluded_keys.contains(&key_string)
        };

        if should_randomize {
            generated_colours_map.insert(key_string, generate_random_colour_hex());
        } else {
            if let Some(current_hex) = current_colours.get(key) {
                generated_colours_map.insert(key_string, current_hex.clone());
            } else {
                eprintln!("Warning: Colour key '{}' not found in current kitty.conf. Defaulting to #000000.", key);
                generated_colours_map.insert(key_string, "000000".to_string());
            }
        }
    }

    println!("\nGenerated new random colours:");
    update_kitty_config_with_colours(config_file_path, &generated_colours_map)?;

    println!("\nKitty colours updated in config file!");
    println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");
    Ok(())
}

pub fn print_current_colours_to_terminal(config_file_path: &PathBuf) -> Result<(), io::Error> {
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

pub fn shuffle_current_colours(
    config_file_path: &PathBuf,
    exception_keys_input: &Option<String>,
    force_keys_input: &Option<String>,
) -> Result<(), io::Error> {
    println!("Shuffling current colours...");

    let current_colours_map = extract_current_colours(config_file_path)?;

    let forced_keys = parse_color_keys_input(force_keys_input);
    let excluded_keys = parse_color_keys_input(exception_keys_input);

    let mut shufflable_keys_full_names: Vec<String> = Vec::new();
    let mut fixed_colours_map: HashMap<String, String> = HashMap::new();

    for &key in COLOUR_KEYS.iter() {
        let key_string = key.to_string();

        let should_be_shuffled = if !forced_keys.is_empty() {
            forced_keys.contains(&key_string)
        } else {
            !excluded_keys.contains(&key_string)
        };

        if should_be_shuffled {
            if let Some(_colour_hex) = current_colours_map.get(key) {
                shufflable_keys_full_names.push(key_string);
            } else {
                eprintln!("Warning: Colour key '{}' not found in current kitty.conf for shuffling. It will be ignored for shuffling.", key);
            }
        } else {
            if let Some(colour_hex) = current_colours_map.get(key) {
                fixed_colours_map.insert(key_string, colour_hex.clone());
            } else {
                eprintln!("Warning: Colour key '{}' not found in current kitty.conf. It will be treated as #000000 and fixed.", key);
                fixed_colours_map.insert(key_string, "000000".to_string());
            }
        }
    }

    if shufflable_keys_full_names.len() < 2 {
        eprintln!("Warning: Not enough eligible colours (less than 2) to perform a meaningful shuffle. No changes applied.");
        return Ok(());
    }

    let mut shufflable_hex_values: Vec<String> = shufflable_keys_full_names
        .iter()
        .map(|key_full_name| {
            current_colours_map.get(key_full_name).cloned().unwrap_or_else(|| "000000".to_string())
        })
        .collect();


    let mut rng = rand::rng();
    shufflable_hex_values.shuffle(&mut rng);

    let mut shuffled_colours_map: HashMap<String, String> = HashMap::new();
    let mut shufflable_idx = 0;

    for &key in COLOUR_KEYS.iter() {
        let key_string = key.to_string();

        if fixed_colours_map.contains_key(&key_string) {
            shuffled_colours_map.insert(key_string.clone(), fixed_colours_map[&key_string].clone());
        } else {
            if shufflable_idx < shufflable_hex_values.len() {
                 shuffled_colours_map.insert(key_string, shufflable_hex_values[shufflable_idx].clone());
                 shufflable_idx += 1;
            } else {
                eprintln!("Internal Warning: Missing shuffled value for key '{}'. Defaulting to #000000.", key);
                shuffled_colours_map.insert(key_string, "000000".to_string());
            }
        }
    }

    update_kitty_config_with_colours(config_file_path, &shuffled_colours_map)?;

    println!("\nKitty colours shuffled and updated in config file!");
    println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");

    Ok(())
}