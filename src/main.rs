use crossterm::terminal::ClearType;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{cursor, event, execute, terminal};
use std::io::stdout;
use std::time::Duration;

struct Output{
    win_size: (usize, usize)
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap(); 
        Self { win_size }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All));
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&self) {
        let screen_rows = self.win_size.1;
        for i in 0..screen_rows {
            print!("->");
            if i < screen_rows - 1 {
                println!("\r")
            }
            // stdout().flush();
        }
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}
struct Reader;

/* add the following*/
impl Reader {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
            else {
                println!("Type input\r");
            }
        }
    }
}

struct Editor {
    reader: Reader,
    output: Output
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader, output: Output::new() }
    }

    fn process_keypress(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('Q'),
                modifiers: event::KeyModifiers::SHIFT,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}
fn main()-> crossterm::Result<()> {
    terminal::enable_raw_mode().expect("Error in enable raw mode");
    let editor = Editor::new();
    while editor.run()? {}
    Ok(())

}