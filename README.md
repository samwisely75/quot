# Quot

[![License](https://img.shields.io/badge/license-Elastic%20License%202.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/github/v/release/samwisely75/quot)](https://github.com/samwisely75/quot/releases)
[![Crates.io](https://img.shields.io/crates/v/quot.svg)](https://crates.io/crates/quot)
[![CI](https://github.com/samwisely75/quot/actions/workflows/ci.yml/badge.svg)](https://github.com/samwisely75/quot/actions/workflows/ci.yml)
[![Release](https://github.com/samwisely75/quot/actions/workflows/release.yml/badge.svg)](https://github.com/samwisely75/quot/actions/workflows/release.yml)

A fast and flexible Rust command-line tool that converts text input into escaped string literals with support for multiple quote styles. Perfect for developers who need to quickly escape text for use in code.

- **Multiple Input Methods**: Interactive keyboard input, piped input, file input, or clipboard input
- **Clipboard Support**: Direct text processing from system clipboard with `-c/--clipboard` flag
- **Multiple Quote Styles**: Double quotes, single quotes, or raw strings (Rust-style)
- **Fast & Lightweight**: Built in Rust for optimal performance
- **Cross-Platform**: Works on macOS, Linux, and Windows

## Usage

### Basic Usage

```bash
# Interactive mode
quot

# Read from file
quot -f input.txt

# Read from piped input
cat file.txt | quot

# Read from system clipboard
quot -c
```

### Quote Style Options

```bash
quot -f input.txt
# Output: "Hello \"world\"\nLine 2"

quot -m single -f input.txt
# Output: 'Hello "world"\nLine 2'

quot -m raw -f input.txt
# Output: r#"Hello "world"
# Line 2"#
```

### Quote Style Comparison

| Style | Flag | Escapes | Use Case |
|-------|------|---------|----------|
| Double | `-m double` (default) | `\"`, `\\`, `\n`, `\r`, `\t` | General purpose, most languages |
| Single | `-m single` | `\'`, `\\`, `\n`, `\r`, `\t` | Languages that prefer single quotes |
| Raw | `-m raw` | None (raw strings) | Rust code, regex patterns, paths |

### Interactive Mode

When you run `quot` without arguments and input isn't piped, you enter interactive mode:

```text
 1> Hello world
 2> This is line 2
 3> Special chars: "quotes" and \backslashes
 4> 
"Hello world\nThis is line 2\nSpecial chars: \"quotes\" and \\backslashes\n"
```

### Clipboard Support

The `--clipboard` (or `-c`) flag allows you to process text directly from your system clipboard:

```bash
# Copy some text to clipboard first, then:
quot --clipboard                # Double quotes (default)
quot -c -m single               # Single quotes  
quot -c -m raw                  # Raw strings

# Example workflow:
# 1. Copy this multi-line text to clipboard:
#    Hello "world"
#    Line with tab:    here
#    Backslash: \test
# 2. Run: quot --clipboard
# 3. Output: "Hello \"world\"\nLine with tab:\there\nBackslash: \\test"
```

The clipboard feature works on all supported platforms (Windows, macOS, Linux) and handles multi-line content seamlessly.

## Installation

### macOS

```bash
# install
brew tap samwisely75/tap
brew install quot

# update to latest version
brew upgrade quot
```

### Linux

```bash
# Debian/Ubuntu:
sudo dpkg -i quot_VERSION_amd64.deb

# RHEL/CentOS/Fedora:**
sudo rpm -ivh quot-VERSION-1.x86_64.rpm
```

### Windows

```powershell
# Download the latest release asset (replace the asset name if needed)
Invoke-WebRequest -Uri "https://github.com/samwisely75/quot/releases/latest/download/quot-windows-x64.exe" -OutFile "quot.exe"

# (Optional) If the release is a zip file, download and extract:
# Invoke-WebRequest -Uri "https://github.com/samwisely75/quot/releases/latest/download/quot-windows-x64.zip" -OutFile "quot.zip"
# Expand-Archive -Path "quot.zip" -DestinationPath "quot"

# Run the binary
.\quot.exe --help
```

## Why Quot?

Quot fills a specific niche for developers who frequently need to escape text for use in code:

- **Developer-focused**: Designed specifically for code generation and string literal creation
- **Multiple quote styles**: Unlike basic escape tools, supports different quoting conventions
- **Interactive editing**: Full-featured line editing with visual feedback
- **Advanced clipboard paste**: Seamlessly handles multi-line clipboard content - a complex feature that sets Quot apart
- **Raw string support**: Intelligent raw string generation for complex content
- **Multi-input flexibility**: Works seamlessly with files, pipes, or interactive input
- **Familiar name**: Inspired by the HTML `&quot;` entity that developers know well

## License

This project is licensed under the Elastic License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
