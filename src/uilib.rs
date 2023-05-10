#[allow(unused_imports)]
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Frame, Terminal,
};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block1 = Block::default().title("Block").borders(Borders::ALL);
    let block2 = Block::default().title("Block 2").borders(Borders::ALL);

    f.render_widget(block1, chunks[0]);
    f.render_widget(block2, chunks[2]);
}
