# Rod's Terminal Colours (rtc)

`rtc` is a CLI tool designed to manage and personalize your terminal colour schemes. It provides functionalities to generate random colour schemes, create backups, load saved schemes, print current colours, and shuffle existing colours. The current version only supports the Kitty terminal emulator.

## Features

* **Random Colour Generation (`-r`, `--random`)**: Apply a new random colour scheme.
* **Colour Backup (`-b`, `--backup`)**: Save your current Kitty colours to a dedicated backup file.
* **Colour Load (`-l`, `--load`)**: Restore Kitty's colours from a previously saved backup file.
* **Print Current Colours (`-c`, `--colours`)**: Display the active colours directly in your terminal.
* **Shuffle Colours (`-s`, `--shuffle`)**: Rearrange your currently applied Kitty colours into a new random order.
* **Named Backups (`-n <NAME>`, `--name <NAME>`)**: Use custom names for your backup and load operations.

## Installation

To install `rtc`, you need to have the Rust programming language and its package manager, Cargo, installed on your system. If not, you can install Rust via [rustup.rs](https://rustup.rs/).

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/lordofhunger/rods-terminal-colours](https://github.com/lordofhunger/rods-terminal-colours) # Replace with your actual repository URL
    ```

2.  **Navigate into the project directory:**
    ```bash
    cd rods-terminal-colours/rtc
    ```

3.  **Build the project in release mode (for optimized performance):**
    ```bash
    cargo build --release
    ```

4.  **Install the executable to a directory in your system's PATH:**
    This makes `rtc` globally accessible from any terminal. You might be prompted for your sudo password.
    ```bash
    sudo cp target/release/rtc /usr/local/bin/
    ```

5.  **Verify installation:**
    ```bash
    rtc --version
    # Expected output: rtc 0.1.0 (or your current version)
    ```

**Note on Configuration Files:**
`rtc` interacts with your Kitty configuration colours in either `~/.config/kitty/kitty.conf` or `~/.kitty.conf`. It stores its own backup files in `~/.config/rtc/`.


## Usage

After installation, `rtc` can be used directly from your terminal. **Remember to restart Kitty after applying new colours for changes to take effect.**

```bash
# Display help information
rtc --help

# Generate and apply a new random colour scheme
rtc -r

# Create a default backup of your current colours
rtc -b

# Create a named backup of your current colours
rtc -b -n my_awesome_scheme

# Load colours from the default backup
rtc -l

# Load colours from a named backup
rtc -l -n my_awesome_scheme

# Print the currently applied 19 prominent colours
rtc -c

# Shuffle the currently applied 19 prominent colours
rtc -s