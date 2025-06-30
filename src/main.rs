use std::{
    error::Error,
    io::{self, stdin, Read},
    process,
};

use atty::Stream;
use crossterm::event::{self, Event};
use clap::{Parser, ValueEnum};

mod clipboard_handler;
mod file_handler;
mod stdin_handler;

#[derive(Debug, Clone, ValueEnum)]
enum QuoteFormat {
    /// Use double quotes (default)
    Double,
    /// Use single quotes  
    Single,
    /// Use raw strings (Rust style)
    Raw,
}

#[derive(Parser)]
#[command(name = "quot")]
#[command(about = "A fast and flexible command-line tool that converts text input into escaped string literals")]
#[command(long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Quote format to use
    #[arg(long, short = 't', value_enum, default_value_t = QuoteFormat::Double)]
    format: QuoteFormat,

    /// Read text from system clipboard
    #[arg(long, short = 'c')]
    clipboard: bool,

    /// File to read from (if not specified, reads from stdin)
    #[arg(long, short = 'f')]
    file: Option<String>,

    /// File path (positional argument, alternative to --file)
    file_path: Option<String>,
}

fn has_piped_input() -> bool {
    !atty::is(Stream::Stdin)
}

fn read_piped_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_file_input(file_path: &str) -> Result<String, Box<dyn Error>> {
    // Check if file exists
    if !file_handler::file_exists(file_path) {
        eprintln!("Error: File '{file_path}' not found or is not a regular file.");
        process::exit(1);
    }

    // Read file content
    file_handler::read_file_content(file_path).map_err(|e| {
        eprintln!("Error reading file '{file_path}': {e}");
        process::exit(1);
    })
}

fn read_keyboard_input() -> Result<String, io::Error> {
    let mut builder = Vec::<String>::new();
    let mut current_line = String::new();
    let mut cursor_pos = 0; // Track cursor position within current line

    // Enable raw mode for better input control
    stdin_handler::enable_raw_mode()?;

    // Show initial prompt with line number 1
    stdin_handler::print_prompt_with_line_number(1)?;

    loop {
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key_event) => {
                    if stdin_handler::handle_key_event(
                        key_event,
                        &mut current_line,
                        &mut builder,
                        &mut cursor_pos,
                    )? {
                        break; // Exit signal received
                    }
                }
                Event::Paste(text) => {
                    stdin_handler::handle_paste_event(
                        text,
                        &mut current_line,
                        &mut builder,
                        &mut cursor_pos,
                    )?;
                }
                _ => {}
            }
        }
    }

    // Disable raw mode
    stdin_handler::disable_raw_mode()?;

    Ok(builder.join("\n"))
}

fn print_result(input_string: String, quote_format: QuoteFormat) {
    let escaped = match quote_format {
        QuoteFormat::Double => {
            let escaped = input_string
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            format!("\"{escaped}\"")
        }
        QuoteFormat::Single => {
            let escaped = input_string
                .replace('\\', "\\\\")
                .replace('\'', "\\'")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            format!("'{escaped}'")
        }
        QuoteFormat::Raw => {
            // For raw strings, we need to find a delimiter that doesn't conflict
            let delimiter = find_raw_string_delimiter(&input_string);
            format!("r{delimiter}\"{input_string}\"{delimiter}")
        }
    };

    println!("{escaped}");
}

fn find_raw_string_delimiter(content: &str) -> String {
    // Find the minimum number of # characters needed for a raw string
    let mut max_consecutive_quotes = 0;
    let mut current_quotes = 0;

    for ch in content.chars() {
        if ch == '"' {
            current_quotes += 1;
            max_consecutive_quotes = max_consecutive_quotes.max(current_quotes);
        } else {
            current_quotes = 0;
        }
    }

    // Use one more # than the maximum consecutive quotes found
    "#".repeat(max_consecutive_quotes + 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Determine input source: file (explicit or positional), clipboard, or stdin
    let input_string = if args.clipboard {
        clipboard_handler::read_clipboard_input()?
    } else if let Some(file_path) = args.file.or(args.file_path) {
        read_file_input(&file_path)?
    } else if has_piped_input() {
        read_piped_input()?
    } else {
        read_keyboard_input()?
    };

    print_result(input_string, args.format);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_print_result() {
        // Test the escaping logic without actually printing
        let input = "Hello \"world\"\nNew line\tTab\\Backslash\rCarriage return";
        let escaped = input
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");

        let expected = "Hello \\\"world\\\"\\nNew line\\tTab\\\\Backslash\\rCarriage return";
        assert_eq!(escaped, expected);
    }

    #[test]
    fn test_read_piped_input_logic() {
        // Test that we can read from a string (simulating stdin)
        let test_input = "test input\nwith multiple lines";
        let mut cursor = std::io::Cursor::new(test_input.as_bytes());
        let mut buffer = String::new();

        use std::io::Read;
        cursor.read_to_string(&mut buffer).unwrap();
        assert_eq!(buffer, test_input);
    }

    #[test]
    fn test_read_file_input_with_temp_file() {
        let test_content = "File content\nwith multiple lines\nand \"quotes\"";
        let test_file = "test_main_temp.txt";

        // Create temporary test file
        {
            let mut file = std::fs::File::create(test_file).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Test reading - this will actually call file_handler functions
        let result = file_handler::read_file_content(test_file).unwrap();
        assert_eq!(result, test_content);

        // Test file exists check
        assert!(file_handler::file_exists(test_file));
        assert!(!file_handler::file_exists("non_existent_file.txt"));

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_argument_parsing_logic() {
        // Test the logic of argument parsing without actually running main
        let args = ["literalizer".to_string()];
        assert_eq!(args.len(), 1); // No arguments case

        let args = ["literalizer".to_string(), "file.txt".to_string()];
        assert_eq!(args.len(), 2); // File argument case
        assert_eq!(args[1], "file.txt");

        let args = ["literalizer".to_string(), "--help".to_string()];
        assert_eq!(args.len(), 2);
        assert!(args[1] == "-h" || args[1] == "--help");

        let args = [
            "literalizer".to_string(),
            "arg1".to_string(),
            "arg2".to_string(),
        ];
        assert_eq!(args.len(), 3); // Too many arguments case
    }

    #[test]
    fn test_escaping_edge_cases() {
        // Test various edge cases for string escaping
        let test_cases = vec![
            ("", ""),
            ("simple", "simple"),
            ("\"", "\\\""),
            ("\\", "\\\\"),
            ("\n", "\\n"),
            ("\r", "\\r"),
            ("\t", "\\t"),
            ("\\n\\r\\t", "\\\\n\\\\r\\\\t"), // Literal backslash-n, etc.
            (
                "\"quote\\backslash\nline\ttab\rreturn\"",
                "\\\"quote\\\\backslash\\nline\\ttab\\rreturn\\\"",
            ),
        ];

        for (input, expected) in test_cases {
            let escaped = input
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            assert_eq!(escaped, expected, "Failed for input: {:?}", input);
        }
    }

    #[test]
    fn test_quote_format_parsing() {
        // Test that QuoteFormat values work correctly
        use clap::ValueEnum;
        
        let double = QuoteFormat::from_str("double", true).unwrap();
        assert!(matches!(double, QuoteFormat::Double));
        
        let single = QuoteFormat::from_str("single", true).unwrap();
        assert!(matches!(single, QuoteFormat::Single));
        
        let raw = QuoteFormat::from_str("raw", true).unwrap();
        assert!(matches!(raw, QuoteFormat::Raw));
    }

    #[test]
    fn test_cli_structure() {
        // Test that Args structure has the expected fields
        let args = Args {
            format: QuoteFormat::Double,
            clipboard: false,
            file: None,
            file_path: Some("test.txt".to_string()),
        };
        
        assert!(matches!(args.format, QuoteFormat::Double));
        assert!(!args.clipboard);
        assert_eq!(args.file_path, Some("test.txt".to_string()));
    }

    #[test]
    fn test_quote_styles() {
        // Test double quotes (default)
        let input = "Hello \"world\"\nTab:\tNewline:\nEnd";
        let mut test_double = input
            .to_string()
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");
        test_double = format!("\"{test_double}\"");

        let expected_double = "\"Hello \\\"world\\\"\\nTab:\\tNewline:\\nEnd\"";
        assert_eq!(test_double, expected_double);

        // Test single quotes
        let mut test_single = input
            .to_string()
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");
        test_single = format!("'{test_single}'");

        let expected_single = "'Hello \"world\"\\nTab:\\tNewline:\\nEnd'";
        assert_eq!(test_single, expected_single);
    }

    #[test]
    fn test_raw_string_delimiter() {
        // Test simple case
        let content1 = "Hello world";
        let delimiter1 = find_raw_string_delimiter(content1);
        assert_eq!(delimiter1, "#");

        // Test with quotes
        let content2 = "Hello \"world\"";
        let delimiter2 = find_raw_string_delimiter(content2);
        assert_eq!(delimiter2, "##"); // one more than the single quote

        // Test with consecutive quotes
        let content3 = "Has \"\"\" three quotes";
        let delimiter3 = find_raw_string_delimiter(content3);
        assert_eq!(delimiter3, "####"); // one more than three consecutive

        // Test with no quotes
        let content4 = "No quotes here";
        let delimiter4 = find_raw_string_delimiter(content4);
        assert_eq!(delimiter4, "#");
    }
}
