# PS2 Redump Downloader (Rust)

A simple CLI tool to search, download, and extract PlayStation 2 ISOs from the Redump database.

- Fast, minimal, and cross-platform
- Rust port inspired by the original Python tool by juanpomares ([link](https://github.com/juanpomares/PS3-Redump-downloader))

## Quick Start

1. **Install Rust:** https://rustup.rs/
2. **Clone and build:**
   ```bash
   git clone https://github.com/leji-a/ps2-redump-downloader.git
   cd ps2-redump-downloader
   cargo build --release
   ```
3. **Run:**
   ```bash
   ./target/release/ps2-redump-downloader
   ```

## Global Installation

Install globally for easy access from anywhere:

```bash
cargo install --path .
ps2-redump-downloader
```

## Config Example

```ini
[url]
ISO = https://myrient.erista.me/files/Redump/Sony%20-%20PlayStation%202/

[Download]
# Downloaded Game list fileName 
LIST_FILES_JSON_NAME = listPS2Titles.json 

# Download ISO file using navigator
EXTERNAL_ISO = 0 

MAX_RETRIES = 10
DELAY_BETWEEN_RETRIES = 10
TIMEOUT_REQUEST = 600

[folder]
TMP_FOLDER_NAME = ~/PS2-Games
TMP_ISO_FOLDER_NAME = iso_files
```
> You can leave 'TMP_ISO_FOLDER_NAME' empty in case you want the isos in 'TMP_FOLDER_NAME'

## Config File Location

The application looks for `config.ini` in these locations (in order):

- **Current directory:** Where you run the binary or `cargo run` (recommended for development)
- **Linux/macOS:**
  - `~/.config/ps2-redump-downloader/config.ini`
  - `/etc/ps2-redump-downloader/config.ini`
- **Windows:**
  - `%APPDATA%\ps2-redump-downloader\config.ini`
  - `C:\ProgramData\ps2-redump-downloader\config.ini`

## Basic Usage

```
$ ps2-redump-downloader
Find PS2 title to download: grand theft auto 
1. Grand Theft Auto - San Andreas (Europe)
Enter PS2 title number: 1
Downloading... [progress]
Extracting... [progress]
Done!
```

## Download Location

By default, downloaded ISOs are saved to:
- `~/PS2-Games/iso_files/` (Linux/macOS)
- `C:\Users\YourName\PS2-Games\iso_files\` (Windows)

You can change this in the `[folder]` section of `config.ini`.

## Tips
- **Download timeout:** Set with `TIMEOUT_REQUEST` (seconds) in `config.ini` (default: 600 = 10 minutes)
- **Retries:** Set `MAX_RETRIES` and `DELAY_BETWEEN_RETRIES` for failed downloads
- **EXTERNAL_ISO:** Set to `1` to use your browser for downloads instead of the built-in downloader
- **Game list cache:** The game list is cached as `listPS2Titles.json` in your chosen folder

---
