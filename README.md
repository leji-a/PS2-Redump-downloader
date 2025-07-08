# PS2 Redump Downloader

A fast and efficient Rust application for downloading PlayStation 2 games from Redump databases. This tool provides a command-line interface to search, download, and extract PS2 ISO files with progress indicators and resume capability.

## Features

- 🔍 **Search & Filter**: Search PS2 games by title with real-time filtering
- ⬇️ **Resume Downloads**: Automatically resumes interrupted downloads
- 📊 **Progress Indicators**: Visual progress bars for downloads and extraction
- 🗜️ **Auto-Extraction**: Automatically extracts ZIP files to ISO format
- 🔧 **Configurable**: Customizable timeouts, retries, and folder paths
- 🌐 **Cross-Platform**: Works on Windows, macOS, and Linux
- ⚡ **High Performance**: Built in Rust for optimal speed and memory efficiency

## Prerequisites

- **Rust** (1.70 or higher)
- **Internet connection** for downloading games
- **Sufficient disk space** for ISO files (typically 1-4GB per game)

### Installing Rust

#### Windows
1. Download the Rust installer from [https://rustup.rs/](https://rustup.rs/)
2. Run the installer and follow the prompts
3. Restart your terminal/command prompt

#### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

### From Source (Recommended)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/ps2-redump-downloader.git
   cd ps2-redump-downloader
   ```

2. **Build the application**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   # Development
   cargo run
   
   # Or use the compiled binary
   ./target/release/ps2-redump-downloader
   ```

### Install as a global binary (recommended for easy access)

You can install the downloader globally so you can run it from anywhere:

```bash
cargo install --path .
```

After installing, you can run the downloader from any directory:

```bash
ps2-redump-downloader
```

### Cross-Platform Compilation

#### For Windows (from Linux/macOS)
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

#### For macOS (from Linux)
```bash
cargo build --release --target x86_64-apple-darwin
```

## Usage

### Basic Usage

1. **Start the application**:
   ```bash
   cargo run
   ```

2. **Search for games**:
   - Type part of the game title (e.g., "gta", "final fantasy")
   - Press Enter to search
   - The search is case-insensitive and supports partial matches

3. **Select a game**:
   - Choose a game from the filtered list by entering its number
   - The download will start automatically

4. **Monitor progress**:
   - Download progress is shown with a progress bar
   - Extraction progress is shown during ZIP extraction
   - Files are automatically saved to the `tmp/iso_files/` directory

### Example Session

```
PS2 Redump Downloader

Find PS2 title to download: gta
1. Grand Theft Auto - San Andreas (Europe, Australia) (En,Fr,De,Es,It) (v1.03) (2.91 GiB)
2. Grand Theft Auto - Vice City (Europe) (En,Fr,De,Es,It) (v1.40) (1.85 GiB)
3. Grand Theft Auto III (Europe) (En,Fr,De,Es,It) (v1.40) (1.85 GiB)

Enter PS2 title number [1-3]: 1

Selected Grand Theft Auto - San Andreas (Europe, Australia) (En,Fr,De,Es,It) (v1.03)

 # ISO file...
Attempting download from: https://myrient.erista.me/files/Redump/Sony%20-%20PlayStation%202/Grand%20Theft%20Auto%20-%20San%20Andreas%20%28Europe%2C%20Australia%29%20%28En%2CFr%2CDe%2CEs%2CIt%29%20%28v1.03%29.zip
⠲ [00:04:58] [##########################>-------------] 1.92 GiB/2.91 GiB (3m)

Extracting ZIP file...
⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ Extracting files... 00:01:23

Grand Theft Auto - San Andreas (Europe, Australia) (En,Fr,De,Es,It) (v1.03) downloaded :)
```

## Configuration

The application uses a `config.ini` file for configuration. You can modify these settings:

### config.ini

```ini
[url]
# PS2 Redump sources (no decryption needed)
ISO = https://myrient.erista.me/files/Redump/Sony%20-%20PlayStation%202/

[Download]
# Downloaded Game list fileName 
LIST_FILES_JSON_NAME = listPS2Titles.json 

# Download ISO file using navigator (0 = use built-in downloader, 1 = open browser)
EXTERNAL_ISO = 0 

# Retry settings
MAX_RETRIES = 10
DELAY_BETWEEN_RETRIES = 10
TIMEOUT_REQUEST = 600

[folder]
TMP_FOLDER_NAME = tmp
TMP_ISO_FOLDER_NAME = iso_files
```

### Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `ISO` | Base URL for PS2 game downloads | Redump URL |
| `EXTERNAL_ISO` | Use browser download instead of built-in downloader | 0 (built-in) |
| `MAX_RETRIES` | Number of retry attempts for failed downloads | 10 |
| `DELAY_BETWEEN_RETRIES` | Seconds to wait between retries | 10 |
| `TIMEOUT_REQUEST` | Request timeout in seconds | 600 (10 minutes) |
| `TMP_FOLDER_NAME` | Temporary folder name | tmp |
| `TMP_ISO_FOLDER_NAME` | ISO files folder name | iso_files |

## File Structure

```
ps2-redump-downloader/
├── src/                    # Source code
├── tmp/                    # Temporary files (created automatically)
│   ├── iso_files/         # Downloaded ISO files
│   └── listPS2Titles.json # Cached game list
├── config.ini             # Configuration file
├── Cargo.toml            # Rust dependencies
└── README.md             # This file
```

## Troubleshooting

### Common Issues

#### Download Timeout
- **Problem**: Downloads timeout before completing
- **Solution**: Increase `TIMEOUT_REQUEST` in config.ini (default: 600 seconds)

#### ZIP Extraction Errors
- **Problem**: "Could not find central directory end" error
- **Solution**: The download was incomplete. The app will automatically retry.

#### No Games Found
- **Problem**: Search returns no results
- **Solution**: 
  - Check your internet connection
  - Try different search terms
  - Delete `tmp/listPS2Titles.json` to refresh the game list

#### Permission Errors
- **Problem**: Cannot create folders or write files
- **Solution**: 
  - Ensure you have write permissions in the current directory
  - Run as administrator on Windows if needed

### Performance Tips

1. **Use SSD storage** for faster file operations
2. **Stable internet connection** for reliable downloads
3. **Sufficient disk space** (games are typically 1-4GB each)
4. **Close other bandwidth-heavy applications** during downloads

## Building from Source

### Development Setup

1. **Install Rust** (see Prerequisites)
2. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/ps2-redump-downloader.git
   cd ps2-redump-downloader
   ```

3. **Install dependencies**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

### Dependencies

The application uses these main dependencies:

- `reqwest` - HTTP client for downloads
- `scraper` - HTML parsing for game lists
- `indicatif` - Progress bars and UI
- `tokio` - Async runtime
- `zip` - ZIP file extraction
- `configparser` - Configuration file parsing

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Redump** for providing the PS2 game database
- **Rust community** for the excellent ecosystem
- **Myrient** for hosting the game files

## Disclaimer

This tool is for educational and preservation purposes only. Please ensure you comply with your local laws regarding software downloads and usage. Only download games you own or have the right to access.

## Support

If you encounter issues:

1. Check the [Troubleshooting](#troubleshooting) section
2. Search existing [Issues](https://github.com/yourusername/ps2-redump-downloader/issues)
3. Create a new issue with:
   - Your operating system
   - Rust version (`rustc --version`)
   - Error message
   - Steps to reproduce

---

**Note**: This is a Rust port of the original Python PS2 Redump downloader, offering improved performance, better error handling, and cross-platform compatibility. 