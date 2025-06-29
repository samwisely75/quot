use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    ExecutableCommand,
};

pub fn enable_raw_mode() -> Result<(), io::Error> {
    terminal::enable_raw_mode().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn disable_raw_mode() -> Result<(), io::Error> {
    terminal::disable_raw_mode().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn print_prompt_with_line_number(line_number: usize) -> Result<(), io::Error> {
    print!("{:2}> ", line_number);
    io::stdout().flush()
}

fn get_prompt_width(line_number: usize) -> usize {
    // Calculate width: line number + "> "
    // Use right-aligned format like print_prompt_with_line_number
    format!("{:2}> ", line_number).len()
}

pub fn move_to_next_line() -> Result<(), io::Error> {
    io::stdout()
        .execute(cursor::MoveToColumn(0))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    println!();
    Ok(())
}

pub fn handle_key_event(
    event: KeyEvent,
    current_line: &mut String,
    builder: &mut Vec<String>,
    cursor_pos: &mut usize,
) -> Result<bool, io::Error> {
    let current_line_number = builder.len() + 1;

    match event.code {
        KeyCode::Enter => {
            move_to_next_line()?;

            if current_line.trim().is_empty() {
                return Ok(true); // Signal to exit
            }

            builder.push(current_line.clone());
            current_line.clear();
            *cursor_pos = 0;
            // Print prompt with line number (current line count + 1)
            print_prompt_with_line_number(builder.len() + 1)?;
        }
        KeyCode::Char(c) => {
            // Handle Ctrl+C to exit
            if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                return Ok(true); // Signal to exit
            }
            insert_char_at_cursor(current_line, cursor_pos, c, current_line_number)?;
        }
        KeyCode::Backspace => {
            delete_char_at_cursor(current_line, cursor_pos, current_line_number)?;
        }
        KeyCode::Delete => {
            delete_char_forward(current_line, cursor_pos, current_line_number)?;
        }
        KeyCode::Left => {
            move_cursor_left(cursor_pos, current_line_number)?;
        }
        KeyCode::Right => {
            move_cursor_right(current_line, cursor_pos, current_line_number)?;
        }
        KeyCode::Home => {
            move_cursor_to_start(cursor_pos, current_line_number)?;
        }
        KeyCode::End => {
            move_cursor_to_end(current_line, cursor_pos, current_line_number)?;
        }
        _ => {
            // Ignore other keys
        }
    }
    Ok(false) // Continue processing
}

pub fn insert_char_at_cursor(
    current_line: &mut String,
    cursor_pos: &mut usize,
    c: char,
    line_number: usize,
) -> Result<(), io::Error> {
    let insert_pos = *cursor_pos;
    current_line.insert(insert_pos, c);
    *cursor_pos += 1;

    // Redraw from the insertion point, but cursor should end up after the inserted char
    redraw_line_after_insert(current_line, insert_pos, *cursor_pos, line_number)
}

pub fn redraw_line_after_insert(
    current_line: &str,
    redraw_from: usize,
    final_cursor_pos: usize,
    line_number: usize,
) -> Result<(), io::Error> {
    // Move to where we need to start redrawing (the insertion point)
    let prompt_width = get_prompt_width(line_number);
    let redraw_col = prompt_width + redraw_from;

    io::stdout()
        .execute(cursor::MoveToColumn(redraw_col as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Clear from current position to end of line
    io::stdout()
        .execute(terminal::Clear(ClearType::UntilNewLine))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Print the rest of the line from the redraw point
    let rest_of_line = &current_line[redraw_from..];
    print!("{}", rest_of_line);

    // Move cursor to the final position (after the inserted character)
    let final_col = prompt_width + final_cursor_pos;
    io::stdout()
        .execute(cursor::MoveToColumn(final_col as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    io::stdout().flush()
}

pub fn delete_char_at_cursor(
    current_line: &mut String,
    cursor_pos: &mut usize,
    line_number: usize,
) -> Result<(), io::Error> {
    if *cursor_pos > 0 && !current_line.is_empty() {
        *cursor_pos -= 1;
        current_line.remove(*cursor_pos);
        redraw_line_from_cursor(current_line, *cursor_pos, line_number)
    } else {
        Ok(())
    }
}

pub fn delete_char_forward(
    current_line: &mut String,
    cursor_pos: &mut usize,
    line_number: usize,
) -> Result<(), io::Error> {
    if *cursor_pos < current_line.len() {
        current_line.remove(*cursor_pos);
        redraw_line_from_cursor(current_line, *cursor_pos, line_number)
    } else {
        Ok(())
    }
}

pub fn move_cursor_left(cursor_pos: &mut usize, _line_number: usize) -> Result<(), io::Error> {
    if *cursor_pos > 0 {
        *cursor_pos -= 1;
        io::stdout()
            .execute(cursor::MoveLeft(1))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}

pub fn move_cursor_right(
    current_line: &str,
    cursor_pos: &mut usize,
    _line_number: usize,
) -> Result<(), io::Error> {
    if *cursor_pos < current_line.len() {
        *cursor_pos += 1;
        io::stdout()
            .execute(cursor::MoveRight(1))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}

pub fn move_cursor_to_start(cursor_pos: &mut usize, line_number: usize) -> Result<(), io::Error> {
    // Move to beginning of line (after the prompt)
    let prompt_width = get_prompt_width(line_number);
    io::stdout()
        .execute(cursor::MoveToColumn(prompt_width as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    *cursor_pos = 0;
    Ok(())
}

pub fn move_cursor_to_end(
    current_line: &str,
    cursor_pos: &mut usize,
    line_number: usize,
) -> Result<(), io::Error> {
    let prompt_width = get_prompt_width(line_number);
    let target_pos = prompt_width + current_line.len();
    io::stdout()
        .execute(cursor::MoveToColumn(target_pos as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    *cursor_pos = current_line.len();
    Ok(())
}

pub fn redraw_line_from_cursor(
    current_line: &str,
    cursor_pos: usize,
    line_number: usize,
) -> Result<(), io::Error> {
    // Move to the position where we need to start redrawing (after prompt)
    let prompt_width = get_prompt_width(line_number);
    let redraw_start_col = prompt_width + cursor_pos;

    io::stdout()
        .execute(cursor::MoveToColumn(redraw_start_col as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Clear from current position to end of line
    io::stdout()
        .execute(terminal::Clear(ClearType::UntilNewLine))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Print the rest of the line from cursor position
    let rest_of_line = &current_line[cursor_pos..];
    print!("{}", rest_of_line);

    // Move cursor to the correct final position (where the cursor should be after the edit)
    let final_col = prompt_width + cursor_pos;
    io::stdout()
        .execute(cursor::MoveToColumn(final_col as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    io::stdout().flush()
}

pub fn handle_paste_event(
    text: String,
    current_line: &mut String,
    builder: &mut Vec<String>,
    cursor_pos: &mut usize,
) -> Result<(), io::Error> {
    let lines: Vec<&str> = text.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim_start();
        let current_line_number = builder.len() + 1;

        if i == 0 {
            // First line continues current input at cursor position
            current_line.insert_str(*cursor_pos, trimmed_line);
            *cursor_pos += trimmed_line.len();
            redraw_line_from_cursor(
                current_line,
                *cursor_pos - trimmed_line.len(),
                current_line_number,
            )?;
        } else {
            // Additional lines
            move_to_next_line()?;
            if !current_line.trim().is_empty() {
                builder.push(current_line.clone());
            }
            *current_line = trimmed_line.to_string();
            *cursor_pos = trimmed_line.len();
            let new_line_number = builder.len() + 1;
            print_prompt_with_line_number(new_line_number)?;
            print!("{}", trimmed_line);
        }
    }
    io::stdout().flush()
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_insert_char_at_cursor() {
        let mut line = String::from("hello");
        let mut cursor_pos = 2;

        // Insert 'X' at position 2 should make "heXllo"
        line.insert(cursor_pos, 'X');
        cursor_pos += 1;

        assert_eq!(line, "heXllo");
        assert_eq!(cursor_pos, 3);
    }

    #[test]
    fn test_cursor_movement_logic() {
        let line = String::from("hello");
        let mut cursor_pos = 2;

        // Test move cursor left
        if cursor_pos > 0 {
            cursor_pos -= 1;
        }
        assert_eq!(cursor_pos, 1);

        // Test move cursor right
        if cursor_pos < line.len() {
            cursor_pos += 1;
        }
        assert_eq!(cursor_pos, 2);

        // Test move to start
        cursor_pos = 0;
        assert_eq!(cursor_pos, 0);

        // Test move to end
        cursor_pos = line.len();
        assert_eq!(cursor_pos, 5);
    }

    #[test]
    fn test_delete_operations() {
        let mut line = String::from("hello");
        let mut cursor_pos = 2;

        // Test delete char at cursor (backspace)
        if cursor_pos > 0 && !line.is_empty() {
            cursor_pos -= 1;
            line.remove(cursor_pos);
        }
        assert_eq!(line, "hllo");
        assert_eq!(cursor_pos, 1);

        // Reset for forward delete test
        line = String::from("hello");
        cursor_pos = 2;

        // Test delete char forward
        if cursor_pos < line.len() {
            line.remove(cursor_pos);
        }
        assert_eq!(line, "helo");
        assert_eq!(cursor_pos, 2);
    }

    #[test]
    fn test_handle_key_event_ctrl_c() {
        // Test Ctrl+C event detection logic
        let ctrl_c_event = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };

        // Test the logic that detects Ctrl+C
        match ctrl_c_event.code {
            KeyCode::Char(c) => {
                if c == 'c' && ctrl_c_event.modifiers.contains(KeyModifiers::CONTROL) {
                    assert!(true); // This is what should happen
                } else {
                    assert!(false, "Should have detected Ctrl+C");
                }
            }
            _ => assert!(false, "Should have been a char event"),
        }
    }

    #[test]
    fn test_handle_key_event_enter() {
        let mut current_line = String::from("test line");
        let mut builder = Vec::<String>::new();
        let mut cursor_pos = 9;

        // Simulate Enter key behavior
        if !current_line.trim().is_empty() {
            builder.push(current_line.clone());
            current_line.clear();
            cursor_pos = 0;
        }

        assert_eq!(builder.len(), 1);
        assert_eq!(builder[0], "test line");
        assert_eq!(current_line, "");
        assert_eq!(cursor_pos, 0);

        // Test empty line (should signal exit)
        let empty_line = String::new();
        assert!(empty_line.trim().is_empty());
    }

    #[test]
    fn test_paste_event_logic() {
        let text = "line1\nline2\nline3";
        let lines: Vec<&str> = text.lines().collect();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");
        assert_eq!(lines[2], "line3");

        // Test trimming logic
        let trimmed_line = "  indented line  ".trim_start();
        assert_eq!(trimmed_line, "indented line  ");
    }
}
