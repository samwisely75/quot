# Quot

A fast and flexible Rust command-line tool that converts text input into escaped string literals with support for multiple quote styles. Perfect for developers who need to quickly escape text for use in code.

## Quick Example

```bash
# Input: Hello "world"
# Output: "Hello \"world\""
echo 'Hello "world"' | quot

# Different quote styles
echo 'Hello "world"' | quot --single    # 'Hello "world"'
echo 'Hello "world"' | quot --raw       # r#"Hello "world""#
```

## Features

- 🚀 **Multiple Input Methods**: Interactive keyboard input, piped input, or file input
- 📝 **Interactive Editing**: Full cursor movement, insert/delete, and paste support
- � **Smart Clipboard Paste**: Seamlessly handles multi-line clipboard content with proper formatting
- �📊 **Line Numbers**: Visual line numbering in interactive mode for multi-line input
- 🎨 **Multiple Quote Styles**: Double quotes, single quotes, or raw strings (Rust-style)
- ⚡ **Fast & Lightweight**: Built in Rust for optimal performance
- 🧪 **Well Tested**: Comprehensive test suite with 16+ tests

## Installation

### Using Cargo (Recommended)

If you have Rust installed, you can install `quot` directly from [crates.io](https://crates.io/crates/quot):

```bash
cargo install quot
```

This will download, compile, and install the latest version. The binary will be available in your `$PATH` as `quot`.

**Update to latest version:**

```bash
cargo install quot --force
```

### Pre-built Binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/blueeaglesam/quot/releases):

#### Windows

- **Windows x64**: `quot-windows-x64.exe`
- **Windows ARM64**: `quot-windows-arm64.exe`

#### macOS

- **Intel Macs**: `quot-macos-x64`
- **Apple Silicon (M1/M2)**: `quot-macos-arm64`

#### Linux

- **Linux x64**: `quot-linux-x64`
- **Linux ARM64**: `quot-linux-arm64`

**Quick install script (Unix systems):**

```bash
# Detect platform and download latest release
curl -s https://api.github.com/repos/elasticsatch/quot/releases/latest \
  | grep browser_download_url \
  | grep $(uname -s | tr '[:upper:]' '[:lower:]') \
  | cut -d '"' -f 4 \
  | xargs curl -L -o quot && chmod +x quot
```

### From Source

```bash
git clone https://github.com/elasticsatch/quot
cd quot
cargo build --release
```

The binary will be available at `target/release/quot`.

## Usage

### Basic Usage

```bash
# Interactive mode (shows line numbers)
quot

# Read from file
quot input.txt

# Read from piped input
echo "Hello world" | quot
cat file.txt | quot
```

### Quote Style Options

#### Double Quotes (Default)

```bash
quot input.txt
# Output: "Hello \"world\"\nLine 2"

echo "Test input" | quot --double
# Output: "Test input\n"
```

#### Single Quotes

```bash
quot --single input.txt
# Output: 'Hello "world"\nLine 2'

echo "Test input" | quot --single
# Output: 'Test input\n'
```

#### Raw Strings (Rust-style)

```bash
quot --raw input.txt
# Output: r#"Hello "world"
# Line 2"#

echo "Test input" | quot --raw
# Output: r#"Test input
# "#
```

### Interactive Mode

When you run `quot` without arguments and input isn't piped, you enter interactive mode:

```text
 1> Hello world
 2> This is line 2
 3> Special chars: "quotes" and \backslashes
 4> 
"Hello world\nThis is line 2\nSpecial chars: \"quotes\" and \\backslashes\n"
```

**Interactive Mode Controls:**

- **Enter**: New line
- **Empty line**: Finish input and output result
- **Ctrl+C**: Exit
- **Arrow keys**: Navigate within current line
- **Home/End**: Jump to beginning/end of line
- **Backspace/Delete**: Remove characters
- **Paste (Ctrl+V)**: Multi-line clipboard paste with intelligent formatting and line numbering

## Examples

### Escaping Code Snippets

**Input file (`code.js`):**

```javascript
function greet(name) {
    console.log("Hello, " + name + "!");
}
```

**Command:**

```bash
quot code.js
```

**Output:**

```text
"function greet(name) {\n    console.log(\"Hello, \" + name + \"!\");\n}"
```

### Working with Raw Strings

**Input:**

```text
This has "multiple" quotes like """this""" example.
```

**Command:**

```bash
echo 'This has "multiple" quotes like """this""" example.' | quot --raw
```

**Output:**

```text
r####"This has "multiple" quotes like """this""" example.
"####
```

The tool automatically determines the correct number of `#` characters needed to avoid conflicts.

### Smart Clipboard Paste

One of Quot's standout features is its intelligent clipboard paste handling in interactive mode:

```bash
# Copy this multi-line text to your clipboard:
# function example() {
#     console.log("Hello world!");
#     return true;
# }

# Then run quot and paste with Ctrl+V
quot
# 1> [Paste your code here]
# 2> function example() {
# 3>     console.log("Hello world!");
# 4>     return true;
# 5> }
# 6> 
# Output: "function example() {\n    console.log(\"Hello world!\");\n    return true;\n}"
```

**Paste Features:**

- **Multi-line support**: Paste entire code blocks or text files
- **Automatic line numbering**: Each pasted line gets proper line numbers
- **Smart formatting**: Preserves indentation and structure
- **Cross-platform**: Works on macOS, Linux, and Windows
- **No size limits**: Handle large clipboard content efficiently

### Processing Configuration Files

```bash
# Convert a config file to an escaped string
quot --single config.json

# Chain with other tools
grep "error" log.txt | quot --raw
```

## Quote Style Comparison

| Style | Escapes | Use Case |
|-------|---------|----------|
| `--double` | `\"`, `\\`, `\n`, `\r`, `\t` | General purpose, most languages |
| `--single` | `\'`, `\\`, `\n`, `\r`, `\t` | Languages that prefer single quotes |
| `--raw` | None (raw strings) | Rust code, regex patterns, paths |

## Help

```bash
quot --help
```

```text
Usage:
  quot [OPTIONS] [FILE]    # Read from file
  echo 'text' | quot [OPTIONS]  # Read from stdin (piped)
  quot [OPTIONS]           # Read from stdin (interactive)

Options:
  --double      Use double quotes (default)
  --single      Use single quotes
  --raw         Use raw strings (Rust style)
  -h, --help    Show this help message

Converts input text to an escaped string literal.

Interactive mode:
  Enter empty line or Ctrl+C to finish input
  Line numbers are shown for reference
```

## Development

### Running Tests

```bash
cargo test
```

### Project Structure

```text
src/
├── main.rs           # Main logic, argument parsing, quote styles
├── file_handler.rs   # File input operations
└── stdin_handler.rs  # Interactive keyboard input with line numbers
```

### Dependencies

- `crossterm` - Cross-platform terminal manipulation
- `atty` - TTY detection for piped vs interactive input

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

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
# Activity cleanup - Sun Jun 29 16:45:37 JST 2025
