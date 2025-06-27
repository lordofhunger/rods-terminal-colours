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
    parse_colour_keys_input,
};
mod cli;
use cli::Args;
use clap::Parser;
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
    ].iter().filter(|&&x| x).count();

    if active_modes > 1 {
        eprintln!("Error: Only one main operation (--random, --backup, --load, --get-colours, --shuffle) can be specified at a time.");
        return Ok(());
    }

    let has_exception_keys = args.exception_keys.is_some() && !parse_colour_keys_input(&args.exception_keys).is_empty();
    let has_force_keys = args.force_keys.is_some() && !parse_colour_keys_input(&args.force_keys).is_empty();

    if has_exception_keys && has_force_keys {
        eprintln!("Error: The --exception (-e) and --force (-f) flags cannot be used together. Please choose one.");
        return Ok(());
    }

    if (has_exception_keys || has_force_keys) && !(args.random_colours || args.shuffle) {
        eprintln!("Error: The --exception (-e) or --force (-f) flags can only be used with --random (-r) or --shuffle (-s).");
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
    } else {
        println!("No operation specified.");
        println!("Use `rtc -r` to generate random colours, `rtc -b` to save, or `rtc -l` to load in, `rtc -g` to print current colours, or `rtc -s` to reorder current colours.");
        println!("Add `-n <name>` to specify `backup` or `load` file name for these operations.");
        println!("Use `-e <keys>` with `-r` or `-s` to specify colours to exclude (e.g., `-e bg` or `-e fg,c0`).");
        println!("Use `-f <keys>` with `-r` or `-s` to specify colours to *only* affect (e.g., `-f fg` or `-f bg,c7`). Conflicts with `-e`.");
    }

    Ok(())
}