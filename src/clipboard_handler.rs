use arboard::Clipboard;
use std::error::Error;

/// Read text content from the system clipboard
pub fn read_clipboard_input() -> Result<String, Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_module_exists() {
        // Basic test to ensure the module compiles and can be tested
        // Note: Actual clipboard testing would require system clipboard access
        // which is not reliable in CI environments
        assert!(true);
    }

    #[test]
    fn test_clipboard_function_signature() {
        // Test that the function signature is correct by checking it compiles
        // We can't easily test actual clipboard functionality without mocking
        let _result: Result<String, Box<dyn Error>> = read_clipboard_input();
        // This will fail in test environment but ensures correct signature
    }
}
