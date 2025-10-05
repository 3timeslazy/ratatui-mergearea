use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
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
            .title("Crossterm Minimal Example"),
    );

    loop {
        term.draw(|f| {
            f.render_widget(&textarea, f.area());
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input { key: Key::Char('a'), ctrl: true, .. } => {
              textarea.select_all();
            },
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
