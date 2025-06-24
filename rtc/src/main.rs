use rand::Rng;
use clap::Parser;
use std::{
    collections::HashMap,
    fs,
    io,
    path::PathBuf,
};
use dirs;

const COLOR_KEYS: [&str; 19] = [
    "foreground", "background", "cursor",
    "color0", "color1", "color2", "color3", "color4", "color5", "color6", "color7",
    "color8", "color9", "color10", "color11", "color12", "color13", "color14", "color15",
];

#[derive(Parser, Debug)]
#[command(name = "rtc", author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    randomize: bool,
}

fn generate_random_color_hex() -> String {
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

fn apply_random_kitty_colors() -> Result<(), io::Error> {
    println!("Searching for kitty.conf...");

    let config_file_path = match find_kitty_config_path() {
        Some(path) => {
            println!("Found kitty.conf at: {}", path.display());
            path
        },
        None => {
            eprintln!("Error: kitty.conf not found.");
            eprintln!("Please ensure your kitty.conf is located in ~/.config/kitty/kitty.conf or ~/.kitty.conf");
            return Err(io::Error::new(io::ErrorKind::NotFound, "kitty.conf not found"));
        }
    };

    let main_config_backup_path = config_file_path.with_extension("conf.bak");
    if config_file_path.exists() {
        println!("Creating a backup of your current config to: {}", main_config_backup_path.display());
        fs::copy(&config_file_path, &main_config_backup_path)?;
    }

    let mut generated_colors: HashMap<&str, String> = HashMap::new();
    for &key in COLOR_KEYS.iter() {
        generated_colors.insert(key, generate_random_color_hex());
    }

    println!("\nGenerated new random colors:");

    let original_content = fs::read_to_string(&config_file_path)?;
    let mut new_content_lines = Vec::new();

    for line in original_content.lines() {
        let mut line_modified = false;
        for &key in COLOR_KEYS.iter() {
            if line.trim_start().starts_with(key) && line.contains('#') {
                if let Some(hash_pos) = line.find('#') {
                    let prefix = &line[0..hash_pos];
                    new_content_lines.push(format!("{} #{}\n", prefix.trim_end(), generated_colors[key]));
                    line_modified = true;
                    break;
                }
            }
        }
        if !line_modified {
            new_content_lines.push(format!("{}\n", line));
        }
    }

    let final_content = new_content_lines.join("");

    println!("Writing updated colors directly to: {}", config_file_path.display());
    fs::write(&config_file_path, final_content)?;

    println!("\n🎉 Kitty colors updated in config file!");
    println!("🔔 Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    if args.randomize {
        apply_random_kitty_colors()?;
    } else {
        println!("No operation specified.");
        println!("Use `rtc --randomize` or `rtc -r` to generate colors.");
    }

    Ok(())
}
