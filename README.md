# CSV Viewer Rust 🦀📊

A fast, lightweight, and user-friendly CSV file viewer built with Rust and the egui framework. This desktop application provides an intuitive interface for browsing and viewing CSV data with a clean, modern GUI.

## Features ✨

- **File Browser Integration**: Built-in file dialog to easily select CSV files from any directory
- **Clean Table Display**: View CSV data in a well-organized, scrollable table format
- **Smart Content Handling**: 
  - Automatic truncation of long cell content with hover tooltips to view full text
  - Row and column counting display
  - Striped table rows for better readability
- **Error Handling**: Clear error messages for file loading issues
- **Responsive UI**: Smooth scrolling for both horizontal and vertical navigation
- **Cross-platform**: Works on Windows, macOS, and Linux

## Screenshots 📸

*Main interface with welcome screen*
*CSV data display with table view*

## Technologies Used 🛠️

- **Rust** - Systems programming language for performance and safety
- **egui** - Immediate mode GUI framework
- **eframe** - Framework for building egui applications
- **csv** - CSV parsing library
- **rfd** - Native file dialogs

## Requirements 📋

- Rust 1.70+ (2021 edition)
- Cargo package manager

## Installation & Usage 🚀

### Option 1: Clone and Run

```bash
# Clone the repository
git clone https://github.com/yourusername/csv_viewer_rust.git
cd csv_viewer_rust

# Run the application
cargo run
```

### Option 2: Build Release Version

```bash
# Build optimized release version
cargo build --release

# Run the compiled binary
./target/release/csv-viewer
```

## How to Use 📚

1. **Launch the Application**: Run the program using one of the methods above
2. **Open CSV File**: Click the "📁 Open CSV File" button
3. **Select File**: Choose your CSV file from the file dialog
4. **View Data**: Browse your data in the table view
   - Scroll horizontally and vertically as needed
   - Hover over truncated cells to see full content
   - View row and column counts at the top

## Project Structure 📁

```
csv_viewer_rust/
├── src/
│   └── main.rs          # Main application code
├── cargo.toml           # Project dependencies and metadata
└── README.md           # This file
```

## Dependencies 📦

```toml
[dependencies]
eframe = "0.24"    # Application framework
egui = "0.24"      # GUI library
csv = "1.3"        # CSV parsing
rfd = "0.12"       # File dialogs
```

## Contributing 🤝

Contributions are welcome! Here are some ways you can contribute:

- Report bugs or suggest features via issues
- Submit pull requests with improvements
- Add support for additional file formats
- Improve the user interface
- Add data filtering and searching capabilities

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test thoroughly
4. Commit your changes: `git commit -m 'Add some feature'`
5. Push to the branch: `git push origin feature-name`
6. Submit a pull request

## Future Enhancements 🔮

- [ ] Search and filter functionality
- [ ] Column sorting capabilities
- [ ] Export to different formats
- [ ] Dark/Light theme toggle
- [ ] Column resizing
- [ ] Data type detection and formatting
- [ ] Memory optimization for large files
- [ ] Undo/Redo functionality for data editing

## License 📄

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments 🙏

- Built with the excellent [egui](https://github.com/emilk/egui) immediate mode GUI library
- CSV parsing provided by the [csv](https://github.com/BurntSushi/rust-csv) crate
- File dialogs handled by [rfd](https://github.com/PolyMeilex/rfd)

---

*Made with ❤️ and Rust*
