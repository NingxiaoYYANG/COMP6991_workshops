use termgame::{Controller, CharChunkMap, GameSettings, Game, GameEvent, SimpleEvent, KeyCode, run_game};

use std::error::Error;
use std::time::Duration;
use std::fs;


/// This is a single "buffer".
/// // hint: change the attributes in this struct 
struct Buffer {
    text: String
}



impl Buffer {
    /// This creates a new Buffer, to use it you should run:
    /// ```rust
    /// Buffer::new()
    /// ```
    fn new() -> Buffer {
        Buffer {
            text: String::new()
        }
    }

    /// A [`CharChunkMap`] is how termgame stores characters.
    /// This converts a buffer into something which can be shown on screen.
    /// You will likely not need to change this function.
    fn chunkmap_from_textarea(&mut self, map: &mut CharChunkMap) {
        let (mut line, mut col) = (0, 0);
        for c in self.text.chars() {
            map.insert(col, line, c.into());
            col += 1;
            if c == '\n' {
                line += 1;
                col = 0;
            }
        }
    }

    /// Adds a char to the end of the buffer.
    fn push_char(&mut self, c: char) {
        self.text.push(c);
    }

    /// Removes the last char in the buffer.
    fn pop_char(&mut self) {
        self.text.pop();
    }

    /// Saves the buffer content to a file.
    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let s = format!("{}.txt", filename);
        fs::write(s.as_str(), &self.text)?;
        // println!("{:?}", &self.text);
        Ok(())
    }

    /// Loads content from a file into the buffer.
    fn load_from_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.text = fs::read_to_string(filename)?;
        Ok(())
    }

    // /// This is an example of a function that takes the Buffer as owned,
    // /// as well as another text area; and returns a new Buffer.
    // /// You would either need to return a `Buffer`, or be sure that
    // /// the user will not want the `Buffer` anymore.
    // fn example_owned(self, _another_arg: Buffer) -> Buffer {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// mutable reference.
    // fn example_ref_mut(&mut self, another_arg: i32) {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// reference.
    // fn example_ref(&self) -> i32 {
    //     todo!()
    // }

}

/// This struct implements all the
/// logic for how the editor should work. It
/// implements "Controller", which defines how
/// something should interact with the terminal.
struct BufferEditor {
    buffer: Buffer,
    filename: Option<String>,
}

impl Controller for BufferEditor {
    /// This gets run once, you can probably ignore it.
    fn on_start(&mut self, game: &mut Game) {
        let mut chunkmap = CharChunkMap::new();
        self.buffer.chunkmap_from_textarea(&mut chunkmap);
        game.swap_chunkmap(&mut chunkmap);
    }

    /// Any time there's a keypress, you'll get this
    /// function called.
    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(c)) => {
                self.buffer.push_char(c)
            },
            SimpleEvent::Just(KeyCode::Enter) => {
                self.buffer.push_char('\n')
            },
            SimpleEvent::Just(KeyCode::Backspace) => {
                self.buffer.pop_char()
            },
            SimpleEvent::Just(KeyCode::Esc) => {
                // Save before exiting
                let _ = self.save();
                game.end_game();
            },
            SimpleEvent::Just(KeyCode::Up) => {
                let mut viewport = game.get_viewport();
                if viewport.y > 0 {
                    viewport.y -= 1;
                }
                game.set_viewport(viewport)
            },
            SimpleEvent::Just(KeyCode::Down) => {
                let mut viewport = game.get_viewport();
                viewport.y += 1;
                game.set_viewport(viewport)
            },
            _ => {}
        }
        let mut chunkmap = CharChunkMap::new();
        self.buffer.chunkmap_from_textarea(&mut chunkmap);
        game.swap_chunkmap(&mut chunkmap);

    }

    /// This function gets called regularly, so you can use it
    /// for logic that's independent of key-presses like
    /// implementing a "mouse".
    fn on_tick(&mut self, _game: &mut Game) {}
}

impl BufferEditor {
    /// Saves the current buffer to the file.
    fn save(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(filename) = &self.filename {
            self.buffer.save_to_file(filename)?;
        }
        Ok(())
    }
}

fn run_command(cmd: &str)  -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    
    if parts.is_empty() {
        println!("No command entered!");
        return Ok(());
    }

    match parts[0] {
        "open" => {
            if parts.len() < 2 {
                println!("Usage: open <filename>");
                return Ok(());
            }
            
            let filename = parts[1];
            let mut editor = BufferEditor {
                buffer: Buffer::new(),
                filename: Some(filename.to_string()),
            };
            
            // Try to load existing file
            if let Err(_e) = editor.buffer.load_from_file(filename) {
                println!("Creating new file: {}", filename);
                
            }
            
            run_game(
                &mut editor,
                GameSettings::new()
                    .tick_duration(Duration::from_millis(25))
            )?;
        },
        _ => {
            println!("Command not recognised!");
        }
    }

    Ok(())
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Welcome to BuffeRS. ");

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                run_command(&line)?;
                rl.add_history_entry(line.as_str());
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}
