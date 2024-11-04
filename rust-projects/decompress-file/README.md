# Rust Zip File Decompressor

A simple command-line tool written in Rust for decompressing `.zip` files.

## Overview

This project provides a tool to extract `.zip` archives, preserving the directory structure and file permissions (on Unix systems). It iterates through the contents of the `.zip` file and extracts each item to the specified location.

## Features

- Extracts files and directories from `.zip` archives.
- Preserves directory structure and file permissions (on Unix-like systems).
- Prints extraction details, including file size and comments (if any).
- Error handling and informative messages for missing or invalid input.

## Usage

1. Run the project:
    ```bash
    cargo run <zip-file-path>

