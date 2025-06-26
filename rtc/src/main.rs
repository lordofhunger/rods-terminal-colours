mod util;
mod config;
use config::{find_kitty_config_path};
mod colours;
use colours::{
    create_colours_backup,
    load_colours_from_backup,
    print_current_colours_to_terminal,
    apply_random_colours_to_kitty,
    shuffle_current_colours,
    parse_color_keys_input,};
use clap::Parser;
use std::{
    io,
};

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
    #[arg(short = 'r', long = "random", conflicts_with_all = &["backup", "load", "get_colours", "shuffle", "set_colour"])]
    random_colors: bool,

    /// Create a backup of your current Kitty colour configuration (only the 19 prominent colours)
    #[arg(short = 'b', long = "backup", conflicts_with_all = &["random_colors", "load", "get_colours", "shuffle", "set_colour", "exception_keys", "force_keys"])]
    backup: bool,

    /// Load a saved Kitty colour configuration backup
    #[arg(short = 'l', long = "load", conflicts_with_all = &["random_colors", "backup", "get_colours", "shuffle", "set_colour", "exception_keys", "force_keys"])]
    load: bool,

    /// Print the currently applied 19 prominent colours from Kitty's config
    #[arg(short = 'g', long = "get-colours", conflicts_with_all = &["random_colors", "backup", "load", "shuffle", "set_colour", "exception_keys", "force_keys"])]
    get_colours: bool,

    /// Shuffle the currently applied 19 prominent colours in Kitty's config
    #[arg(short = 's', long = "shuffle", conflicts_with_all = &["random_colors", "backup", "load", "get_colours", "set_colour"])]
    shuffle: bool,

    /// Specify a name for the backup or load operation (e.g., 'my_theme').
    /// If not provided, a default backup/load will be used.
    #[arg(short = 'n', long = "name", value_name = "NAME")]
    name: Option<String>,

    /// Specify colour keys to exclude from randomization/shuffling (e.g., 'bg' or '(fg, c0, c7)').
    /// Use with -r or -s. Conflicts with --force.
    #[arg(short = 'e', long = "exception", value_name = "KEYS", conflicts_with = "force_keys")]
    exception_keys: Option<String>,

    /// Specify colour keys to ONLY apply randomization/shuffling to (e.g., 'bg' or '(fg, c0, c7)').
    /// Use with -r or -s. Conflicts with --exception.
    #[arg(short = 'f', long = "force", value_name = "KEYS")]
    force_keys: Option<String>,

    /// Future feature: Set a specific colour key to a specific hex value.
    /// This argument will enable the set-colour mode.
    #[arg(long = "set-colour", conflicts_with_all = &["random_colors", "backup", "load", "get_colours", "shuffle", "exception_keys", "force_keys"], requires = "set_colour_hex")]
    set_colour: bool,

    /// The hex colour value (e.g., #142569) to set when using --set-colour.
    #[arg(long = "hex", value_name = "HEX_CODE")]
    set_colour_hex: Option<String>,

    /// The name of the colour key (e.g., 'foreground' or 'fg') to set when using --set-colour.
    #[arg(long = "colour-name", value_name = "KEY")]
    set_colour_name: Option<String>,
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

    let has_exception_keys = args.exception_keys.is_some() && !parse_color_keys_input(&args.exception_keys).is_empty();
    let has_force_keys = args.force_keys.is_some() && !parse_color_keys_input(&args.force_keys).is_empty();

    if has_exception_keys && has_force_keys {
        eprintln!("Error: The --exception (-e) and --force (-f) flags cannot be used together. Please choose one.");
        return Ok(());
    }

    if (has_exception_keys || has_force_keys) && !(args.random_colors || args.shuffle) {
        eprintln!("Error: The --exception (-e) or --force (-f) flags can only be used with --random (-r) or --shuffle (-s).");
        return Ok(());
    }


    if args.random_colors {
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
        if let (Some(hex_code), Some(colour_name)) = (&args.set_colour_hex, &args.set_colour_name) {
            println!("Future Feature: Setting colour '{}' to '{}'.", colour_name, hex_code);
            println!("This feature is not yet implemented.");
        } else {
            eprintln!("Error: --set-colour requires --hex and --colour-name.");
        }
    }
    else {
        println!("No operation specified.");
        println!("Use `rtc -r` to generate colours, `rtc -b` to save, or `rtc -l` to load in, `rtc -g` to print current colours, or `rtc -s` to reorder current colours.");
        println!("Add `-n <name>` to specify `backup` or `load` file name for these operations.");
        println!("Use `-e <keys>` with `-r` or `-s` to specify colours to exclude (e.g., `-e bg` or `-e fg,c0`).");
        println!("Use `-f <keys>` with `-r` or `-s` to specify colours to *only* affect (e.g., `-f fg` or `-f bg,c7`). Conflicts with `-e`.");
        println!("Future: Use `--set-colour --hex #RRGGBB --colour-name <key>` to set a specific colour.");
    }

    Ok(())
}