mod util;
mod config;
use config::find_kitty_config_path;
mod colours;
use colours::{
    create_colours_backup,
    load_colours_from_backup,
    print_current_colours_to_terminal,
    apply_random_colours_to_kitty,
    shuffle_current_colours,
    update_kitty_config_with_colours,
    parse_colour_keys_input,
    ColourMap,
    COLOUR_KEYS,
};
mod cli;
use cli::Args;
use clap::Parser;
use std::collections::HashMap;
use std::io;


fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let config_file_path = match find_kitty_config_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: kitty.conf not found. Please ensure it's in ~/.config/kitty/kitty.conf or ~/.kitty.kitty.conf");
            return Err(io::Error::new(io::ErrorKind::NotFound, "kitty.conf not found"));
        }
    };

    let active_modes = [
        args.random_colours,
        args.backup,
        args.load,
        args.get_colours,
        args.shuffle,
        args.set_colour,
    ].iter().filter(|&&x| x).count();

    if active_modes > 1 {
        eprintln!("Error: Only one main operation (--random, --backup, --load, --get-colours, --shuffle, --set-colour) can be specified at a time.");
        return Ok(());
    }

    let has_exception_keys = args.exception_keys.is_some() && !parse_colour_keys_input(&args.exception_keys).is_empty();
    let has_force_keys_for_random_shuffle_or_set = args.force_keys.is_some() && !parse_colour_keys_input(&args.force_keys).is_empty();

    if has_exception_keys && has_force_keys_for_random_shuffle_or_set {
        eprintln!("Error: The --exception (-e) and --force (-f) flags cannot be used together. Please choose one.");
        return Ok(());
    }

    if (has_exception_keys || has_force_keys_for_random_shuffle_or_set) && !(args.random_colours || args.shuffle || args.set_colour) {
        eprintln!("Error: The --exception (-e) or --force (-f) flags can only be used with --random (-r), --shuffle (-s), or --set-colour (-c).");
        return Ok(());
    }

    if args.random_colours {
        apply_random_colours_to_kitty(&config_file_path, &args.exception_keys, &args.force_keys)?;
    } else if args.backup {
        create_colours_backup(&config_file_path, args.name)?;
    } else if args.load {
        load_colours_from_backup(&config_file_path, args.name)?;
    } else if args.get_colours {
        print_current_colours_to_terminal(&config_file_path)?;
    } else if args.shuffle {
        shuffle_current_colours(&config_file_path, &args.exception_keys, &args.force_keys)?;
    } else if args.set_colour {
        let keys_str = args.force_keys.as_ref().expect("force_keys is required by clap for --set-colour");
        let hex_values_str = args.hex_values.as_ref().expect("hex_values is required by clap for --set-colour");

        let keys: Vec<String> = keys_str.split(',')
                                        .map(|s| s.trim().to_string())
                                        .filter(|s| !s.is_empty())
                                        .collect();
        let hex_values: Vec<String> = hex_values_str.split(',')
                                                 .map(|s| s.trim().to_string())
                                                 .filter(|s| !s.is_empty())
                                                 .collect();

        if keys.len() != hex_values.len() {
            eprintln!("Error: The number of colour keys ({}) specified with --force (-f) does not match the number of hex values ({}) specified with --hex-values (-h).", keys.len(), hex_values.len());
            return Ok(());
        }

        let mut colours_to_set: ColourMap = HashMap::new();
        for (i, key_alias) in keys.into_iter().enumerate() {
            let hex_code = &hex_values[i];

            if hex_code.len() != 6 || !hex_code.chars().all(|c| c.is_ascii_hexdigit()) {
                eprintln!("Error: Invalid hex code format for '{}'. Must be 6 hexadecimal characters (e.g., '123456').", hex_code);
                return Ok(());
            }

            let full_key_name = colours::COLOUR_KEY_ALIASES.get(key_alias.as_str())
                                                            .map(|&s| s.to_string())
                                                            .unwrap_or(key_alias.clone());

            if !COLOUR_KEYS.contains(&full_key_name.as_str()) {
                eprintln!("Error: Unknown colour key '{}' (or alias '{}'). Please use a valid Kitty colour name (e.g., 'bg', 'fg', 'c0', 'color15').", full_key_name, key_alias);
                return Ok(());
            }

            colours_to_set.insert(full_key_name, hex_code.to_string());
        }

        println!("\nSetting specific colours in Kitty config:");
        for (key, hex) in &colours_to_set {
            println!("  {}: #{}", key, hex);
        }
        update_kitty_config_with_colours(&config_file_path, &colours_to_set)?;

        println!("\nKitty colours updated in config file!");
        println!("Please restart Kitty manually to see the changes, as live reload is not reliably supported by your Kitty version.");

    } else {
        println!("No operation specified.");
        println!("Use `rtc -r` to generate random colours, `rtc -b` to save, or `rtc -l` to load in, `rtc -g` to print current colours, or `rtc -s` to reorder current colours.");
        println!("Add `-n <name>` to specify `backup` or `load` file name for these operations.");
        println!("Use `-e <keys>` with `-r` or `-s` to specify colours to exclude (e.g., `-e bg` or `-e fg,c0`).");
        println!("Use `-f <keys>` with `-r` or `-s` to specify colours to *only* affect (e.g., `-f fg` or `-f bg,c7`). Conflicts with `-e`.");
        println!("Use `-c -f <keys> -h <hex_codes>` to set specific colours (e.g., `-c -f bg,fg -h 000000,FFFFFF`).");
    }

    Ok(())
}