use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::cmp;
use std::io;
use ratatui_mergearea::{Input, Key, MergeArea};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = MergeArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Textarea with Variable Height"),
    );

    loop {
        term.draw(|f| {
            const MIN_HEIGHT: usize = 3;
            let height = cmp::max(textarea.text().as_str().lines().count(), MIN_HEIGHT) as u16 + 2; // + 2 for borders
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(height), Constraint::Min(0)])
                .split(f.area());
            f.render_widget(&textarea, chunks[0]);
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input_emacs(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.text().as_str());
    Ok(())
}
