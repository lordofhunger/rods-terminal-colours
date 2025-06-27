use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rtc",
    author = "Rod",
    version,
    about = "Rod's Terminal Colours for Kitty",
    long_about = "Rod's Terminal Colours (rtc) is a CLI tool to manage different colour functionalities. It allows you to generate random colour schemes, create backups of your current one, load previously saved ones, print current colours, and shuffle existing colours. Colours are applied to ~/.config/kitty/kitty.kitty.conf or ~/.kitty.kitty.conf.",
)]

pub struct Args {
    /// Generate and apply a random Kitty colour scheme
    #[arg(short = 'r', long = "random", conflicts_with_all = &["backup", "load", "get_colours", "shuffle", "set_colour"])]
    pub random_colours: bool,

    /// Create a backup of your current Kitty colour configuration (only the 19 prominent colours)
    #[arg(short = 'b', long = "backup", conflicts_with_all = &["random_colours", "load", "get_colours", "shuffle", "exception_keys", "force_keys", "set_colour", "hex_values"])]
    pub backup: bool,

    /// Load a saved Kitty colour configuration backup
    #[arg(short = 'l', long = "load", conflicts_with_all = &["random_colours", "backup", "get_colours", "shuffle", "exception_keys", "force_keys", "set_colour", "hex_values"])]
    pub load: bool,

    /// Print the currently applied 19 prominent colours from Kitty's config
    #[arg(short = 'g', long = "get-colours", conflicts_with_all = &["random_colours", "backup", "load", "shuffle", "exception_keys", "force_keys", "set_colour", "hex_values"])]
    pub get_colours: bool,

    /// Shuffle the currently applied 19 prominent colours in Kitty's config
    #[arg(short = 's', long = "shuffle", conflicts_with_all = &["random_colours", "backup", "load", "get_colours", "set_colour", "hex_values"])]
    pub shuffle: bool,

    /// Specify a name for the backup or load operation (e.g., 'my_theme').
    /// If not provided, a default backup/load will be used.
    #[arg(short = 'n', long = "name", value_name = "NAME")]
    pub name: Option<String>,

    /// Specify colour keys to exclude from randomization/shuffling (e.g., 'bg' or '(fg, c0, c7)').
    /// Use with -r or -s. Conflicts with --force.
    #[arg(short = 'e', long = "exception", value_name = "KEYS", conflicts_with = "force_keys")]
    pub exception_keys: Option<String>,

    /// Specify colour keys to ONLY apply randomization/shuffling (with -r or -s) OR to set specific colours (with -c).
    /// (e.g., 'bg' or '(fg, c0, c7)'). Conflicts with --exception.
    #[arg(short = 'f', long = "force", value_name = "KEYS")]
    pub force_keys: Option<String>,

    /// Set one or more specific colour keys to specific hex values.
    /// Requires --force (-f) to specify keys and --hex-values (-h) for the colours.
    #[arg(short = 'c', long = "set-colour", conflicts_with_all = &["random_colours", "backup", "load", "get_colours", "shuffle"], requires_all = &["force_keys", "hex_values"])]
    pub set_colour: bool,

    /// Comma-separated list of 6-digit hex colour values (e.g., '123456,ABCDEF') corresponding to --force (-f) keys.
    /// Use with -c/--set-colour.
    #[arg(short = 'h', long = "hex-values", value_name = "HEX_CODES")]
    pub hex_values: Option<String>,
}