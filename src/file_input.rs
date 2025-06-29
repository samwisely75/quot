use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

pub fn read_file_content<P: AsRef<Path>>(file_path: P) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_file_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

pub fn file_exists<P: AsRef<Path>>(file_path: P) -> bool {
    file_path.as_ref().exists() && file_path.as_ref().is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_file_content() {
        // Create a temporary test file
        let test_content = "Hello\nWorld\nTest";
        let test_file = "test_temp.txt";
        
        {
            let mut file = File::create(test_file).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }
        
        // Test reading the file
        let result = read_file_content(test_file).unwrap();
        assert_eq!(result, test_content);
        
        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_file_exists() {
        assert!(!file_exists("non_existent_file.txt"));
        
        // Create a test file
        let test_file = "test_exists.txt";
        File::create(test_file).unwrap();
        
        assert!(file_exists(test_file));
        
        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
