use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, ClearType},
    ExecutableCommand,
};

pub fn read_all_input() -> Result<String, io::Error> {
    let mut builder = Vec::<String>::new();
    let mut current_line = String::new();
    let mut cursor_pos = 0; // Track cursor position within current line

    // Enable raw mode for better input control
    terminal::enable_raw_mode().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Show initial prompt
    print_prompt()?;

    loop {
        if let Ok(event) = event::read() {
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    if handle_key_event(code, &mut current_line, &mut builder, &mut cursor_pos)? {
                        break; // Exit signal received
                    }
                }
                Event::Paste(text) => {
                    handle_paste_event(text, &mut current_line, &mut builder, &mut cursor_pos)?;
                }
                _ => {}
            }
        }
    }

    // Disable raw mode
    terminal::disable_raw_mode().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(builder.join("\n"))
}

fn print_prompt() -> Result<(), io::Error> {
    print!("> ");
    io::stdout().flush()
}

fn move_to_next_line() -> Result<(), io::Error> {
    io::stdout()
        .execute(cursor::MoveToColumn(0))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    println!();
    Ok(())
}

fn handle_key_event(
    code: KeyCode,
    current_line: &mut String,
    builder: &mut Vec<String>,
    cursor_pos: &mut usize,
) -> Result<bool, io::Error> {
    match code {
        KeyCode::Enter => {
            move_to_next_line()?;

            if current_line.trim().is_empty() {
                return Ok(true); // Signal to exit
            }

            builder.push(current_line.clone());
            current_line.clear();
            *cursor_pos = 0;
            print_prompt()?;
        }
        KeyCode::Char(c) => {
            insert_char_at_cursor(current_line, cursor_pos, c)?;
        }
        KeyCode::Backspace => {
            delete_char_at_cursor(current_line, cursor_pos)?;
        }
        KeyCode::Delete => {
            delete_char_forward(current_line, cursor_pos)?;
        }
        KeyCode::Left => {
            move_cursor_left(cursor_pos)?;
        }
        KeyCode::Right => {
            move_cursor_right(current_line, cursor_pos)?;
        }
        KeyCode::Home => {
            move_cursor_to_start(cursor_pos)?;
        }
        KeyCode::End => {
            move_cursor_to_end(current_line, cursor_pos)?;
        }
        _ => {
            // Ignore other keys
        }
    }
    Ok(false) // Continue processing
}

fn insert_char_at_cursor(
    current_line: &mut String,
    cursor_pos: &mut usize,
    c: char,
) -> Result<(), io::Error> {
    let insert_pos = *cursor_pos;
    current_line.insert(insert_pos, c);
    *cursor_pos += 1;

    // Redraw from the insertion point, but cursor should end up after the inserted char
    redraw_line_after_insert(current_line, insert_pos, *cursor_pos)
}

fn redraw_line_after_insert(
    current_line: &String,
    redraw_from: usize,
    final_cursor_pos: usize,
) -> Result<(), io::Error> {
    // Move to where we need to start redrawing (the insertion point)
    let redraw_col = 2 + redraw_from; // 2 for "> " prompt

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
    let final_col = 2 + final_cursor_pos;
    io::stdout()
        .execute(cursor::MoveToColumn(final_col as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    io::stdout().flush()
}

fn delete_char_at_cursor(
    current_line: &mut String,
    cursor_pos: &mut usize,
) -> Result<(), io::Error> {
    if *cursor_pos > 0 && !current_line.is_empty() {
        *cursor_pos -= 1;
        current_line.remove(*cursor_pos);
        redraw_line_from_cursor(current_line, *cursor_pos)
    } else {
        Ok(())
    }
}

fn delete_char_forward(
    current_line: &mut String,
    cursor_pos: &mut usize,
) -> Result<(), io::Error> {
    if *cursor_pos < current_line.len() {
        current_line.remove(*cursor_pos);
        redraw_line_from_cursor(current_line, *cursor_pos)
    } else {
        Ok(())
    }
}

fn move_cursor_left(cursor_pos: &mut usize) -> Result<(), io::Error> {
    if *cursor_pos > 0 {
        *cursor_pos -= 1;
        io::stdout()
            .execute(cursor::MoveLeft(1))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}

fn move_cursor_right(current_line: &String, cursor_pos: &mut usize) -> Result<(), io::Error> {
    if *cursor_pos < current_line.len() {
        *cursor_pos += 1;
        io::stdout()
            .execute(cursor::MoveRight(1))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}

fn move_cursor_to_start(cursor_pos: &mut usize) -> Result<(), io::Error> {
    // Move to beginning of line (after the "> " prompt)
    io::stdout()
        .execute(cursor::MoveToColumn(2))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    *cursor_pos = 0;
    Ok(())
}

fn move_cursor_to_end(current_line: &String, cursor_pos: &mut usize) -> Result<(), io::Error> {
    let target_pos = 2 + current_line.len(); // 2 for "> " prompt
    io::stdout()
        .execute(cursor::MoveToColumn(target_pos as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    *cursor_pos = current_line.len();
    Ok(())
}

fn redraw_line_from_cursor(current_line: &String, cursor_pos: usize) -> Result<(), io::Error> {
    // Move to the position where we need to start redrawing (after "> " prompt)
    let redraw_start_col = 2 + cursor_pos; // 2 for "> " prompt

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
    let final_col = 2 + cursor_pos;
    io::stdout()
        .execute(cursor::MoveToColumn(final_col as u16))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    io::stdout().flush()
}

fn handle_paste_event(
    text: String,
    current_line: &mut String,
    builder: &mut Vec<String>,
    cursor_pos: &mut usize,
) -> Result<(), io::Error> {
    let lines: Vec<&str> = text.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim_start();

        if i == 0 {
            // First line continues current input at cursor position
            current_line.insert_str(*cursor_pos, trimmed_line);
            *cursor_pos += trimmed_line.len();
            redraw_line_from_cursor(current_line, *cursor_pos - trimmed_line.len())?;
        } else {
            // Additional lines
            move_to_next_line()?;
            if !current_line.trim().is_empty() {
                builder.push(current_line.clone());
            }
            *current_line = trimmed_line.to_string();
            *cursor_pos = trimmed_line.len();
            print!("> {}", trimmed_line);
        }
    }
    io::stdout().flush()
}
