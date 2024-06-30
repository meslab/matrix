use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::io::{stdout, Result, Write};
use std::thread;
use std::time::Duration;

const SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%&*";

fn main() -> Result<()> {
    let mut stdout = stdout();
    let (cols, rows) = terminal::size()?;
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let mut rng = rand::thread_rng();
    let mut drops: Vec<Option<usize>> = vec![None; cols as usize];

    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(event) = event::read()? {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        execute!(stdout, cursor::Hide)?;

        for x in 0..cols {
            if rng.gen_ratio(2, cols as u32) {
                drops[x as usize] = Some(0);
            }
        }

        for x in 0..cols {
            if let Some(y) = drops[x as usize] {
                let symbol = SYMBOLS[rng.gen_range(0..SYMBOLS.len())];
                execute!(
                    stdout,
                    cursor::MoveTo(x, y as u16),
                    terminal::Clear(ClearType::CurrentLine),
                    crossterm::style::Print(symbol as char)
                )?;

                if y as u16 >= rows - 1 {
                    drops[x as usize] = None;
                } else {
                    drops[x as usize] = Some(y + 1);
                }
            }
        }

        stdout.flush()?;
        thread::sleep(Duration::from_millis(50));
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
