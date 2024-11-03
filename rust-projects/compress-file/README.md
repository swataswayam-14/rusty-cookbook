# Rust Gzip File Compressor

This Rust program compresses files using the Gzip format. It reads a source file, compresses its contents, and writes the compressed data to a target file. Additionally, it displays the source file size, compressed file size, and the time taken to perform the compression.

## Features

- **File Compression**: Compresses files using Gzip compression.
- **Command-line Arguments**: Accepts source and target file paths as command-line arguments.
- **Execution Time Measurement**: Measures and displays the time taken for compression.
  
## Requirements

- The `flate2` crate (included in `Cargo.toml`)

## Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/swataswayam-14/rusty-cookbook.git

2. Usage

    To run the program, use the following command format:
    
    ```sh
    cargo run <source_file> <target_file>
