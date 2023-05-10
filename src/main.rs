use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();                      //
    let backend = CrosstermBackend::new(stdout);    // Initalize the terminal output
    let mut terminal = Terminal::new(backend)?;     //
    Ok(())
}
