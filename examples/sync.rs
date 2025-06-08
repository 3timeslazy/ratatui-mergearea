use automerge::AutoCommit;
use autosurgeon::{hydrate, reconcile, Hydrate, Reconcile};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use ratatui_mergearea::{Input, Key, TextArea};
use std::io;

fn inactivate(editor: &mut Editor<'_>) {
    editor.textarea.set_cursor_line_style(Style::default());
    editor.textarea.set_cursor_style(Style::default());
    editor.textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .title(" Inactive (^S to switch) "),
    );
}

fn activate(editor: &mut Editor<'_>) {
    editor
        .textarea
        .set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    editor
        .textarea
        .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    editor.textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Active (^R to sync) "),
    );
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut doc1 = AutoCommit::new();
    let state1 = State {
        text: autosurgeon::Text::with_value("Hello, World"),
    };
    reconcile(&mut doc1, state1).unwrap();

    let doc2 = doc1.fork();

    let editor1 = Editor::new(doc1);
    let editor2 = Editor::new(doc2);
    let mut editors = [editor1, editor2];

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref());

    let mut which = 0;
    activate(&mut editors[0]);
    inactivate(&mut editors[1]);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.area());
            for (editor, chunk) in editors.iter().zip(chunks.iter()) {
                f.render_widget(&editor.textarea, *chunk);
            }
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,

            Input {
                key: Key::Char('s'),
                ctrl: true,
                ..
            } => {
                inactivate(&mut editors[which]);
                which = (which + 1) % 2;
                activate(&mut editors[which]);
            }

            Input {
                key: Key::Char('r'),
                ctrl: true,
                ..
            } => {
                let (left, right) = editors.split_at_mut(1);

                let (curr, other) = if which == 0 {
                    (&mut left[0], &mut right[0])
                } else {
                    (&mut right[0], &mut left[0])
                };

                curr.reconcile();
                other.reconcile();

                curr.merge(other);

                // this prevents 'StaleHeads' panic
                other.update_from_doc();
            }

            input => {
                editors[which].textarea.input(input);
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

    Ok(())
}

struct Editor<'a> {
    doc: AutoCommit,
    textarea: TextArea<'a>,
}

#[derive(Hydrate, Reconcile)]
struct State {
    text: autosurgeon::Text,
}

impl Editor<'_> {
    fn new(doc: AutoCommit) -> Self {
        let state: State = hydrate(&doc).unwrap();

        Self {
            doc,
            textarea: TextArea::new(state.text),
        }
    }

    fn text(&self) -> &autosurgeon::Text {
        self.textarea.text()
    }

    fn merge(&mut self, other: &mut Editor) {
        self.doc.merge(&mut other.doc).unwrap();
        self.update_from_doc();
    }

    fn update_from_doc(&mut self) {
        let state: State = hydrate(&self.doc).unwrap();
        self.textarea.set_text(state.text);
    }

    fn reconcile(&mut self) {
        let state = State {
            text: self.text().clone(),
        };
        reconcile(&mut self.doc, &state).unwrap();
    }
}
