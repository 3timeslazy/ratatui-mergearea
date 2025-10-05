# ratatui-mergearea

**ratatui-mergearea** is a simple [Automerge](https://automerge.org/) text editor widget for ratatui based on [tui-textarea](https://github.com/rhysd/tui-textarea)

>[!Caution]
>This is work-in-progress library for another little project of mine, so not everything that works in the original library works here.
>Features get implemented and tested mostly once I need them :)

>[!Warning]
>The API is not 100% compatible with [tui-textarea](https://github.com/rhysd/tui-textarea). As an example, tui-rs support has been dropped,
>and `input()` and `input_without_shortcuts()` methods replaced with `input_emacs()` and `input()` respectively.

>[!Warning]
>Documentation for the crate is intentionally smaller compared to [tui-textarea](https://github.com/rhysd/tui-textarea). So, for usage patterns and advanced configuration feel free to check it

**Features:**

- Multi-line text editor widget with basic operations (insert/delete characters, auto scrolling, ...)
- Emacs-like shortcuts (`C-n`/`C-p`/`C-f`/`C-b`, `M-f`/`M-b`, `C-a`/`C-e`, `C-h`/`C-d`, `C-k`, `M-<`/`M->`, ...)
- Line number
- Cursor line highlight
- Text selection
- Mouse scrolling

## Examples

Running `cargo run --example` in this repository can demonstrate usage of ratatui-mergearea.

### [`sync`](./examples/sync.rs)

```sh
cargo run --example sync
```

https://github.com/user-attachments/assets/b4113b11-7b7d-4ea5-ace6-c872da8887fc

### [`minimal`](./examples/minimal.rs)

```sh
cargo run --example minimal
```

Minimal usage with default Emacs-like mappings.

<img src="https://raw.githubusercontent.com/rhysd/ss/master/tui-textarea/minimal.gif" width=539 height=172 alt="minimal example">

### [`variable`](./examples/variable.rs)

```sh
cargo run --example variable
```

Simple textarea with variable height following the number of lines.

### [`vim`](./examples/vim.rs)

```sh
cargo run --example vim
```

Vim-like modal text editor.

<img src="https://raw.githubusercontent.com/rhysd/ss/master/tui-textarea/vim.gif" width=590 height=156 alt="Vim emulation example">

## Installation

Add `ratatui-mergearea` crate to dependencies in your `Cargo.toml`. This enables crossterm backend support by default.

## Minimal Usage

```rust,ignore
use ratatui_mergearea::TextArea;
use crossterm::event::{Event, read};

let mut term = ratatui::Terminal::new(...);

// Create an empty `TextArea` instance which manages the editor state
let mut textarea = TextArea::default();

// Event loop
loop {
    term.draw(|f| {
        // Get `ratatui::layout::Rect` where the editor should be rendered
        let rect = ...;
        // Render the textarea in terminal screen
        f.render_widget(&textarea, rect);
    })?;

    if let Event::Key(key) = read()? {
        // Your own key mapping to break the event loop
        if key.code == KeyCode::Esc {
            break;
        }
        // `TextArea::input` can directly handle key events from backends and update the editor state
        textarea.input(key);
    }
}

println!("Lines: {:?}", textarea.text().as_str());
```

`TextArea` is an instance to manage the editor state. By default, it disables line numbers and highlights cursor line
with underline.

`&TextArea` reference implements ratatui's `Widget` trait. Render it on every tick of event loop.

`TextArea::input()` receives inputs from tui backends. The method can take key events from backends such as
`crossterm::event::KeyEvent` or `termion::event::Key` directly if the features are enabled. The method handles default
key mappings as well.

Default key mappings are as follows:

| Mappings                                     | Description                               |
|----------------------------------------------|-------------------------------------------|
| `Ctrl+H`, `Backspace`                        | Delete one character before cursor        |
| `Ctrl+D`, `Delete`                           | Delete one character next to cursor       |
| `Ctrl+M`, `Enter`                            | Insert newline                            |
| `Ctrl+K`                                     | Delete from cursor until the end of line  |
| `Ctrl+J`                                     | Delete from cursor until the head of line |
| `Ctrl+U`                                     | Undo                                      |
| `Ctrl+R`                                     | Redo                                      |
| `Ctrl+C`, `Copy`                             | Copy selected text                        |
| `Ctrl+X`, `Cut`                              | Cut selected text                         |
| `Ctrl+Y`, `Paste`                            | Paste yanked text                         |
| `Ctrl+F`, `→`                                | Move cursor forward by one character      |
| `Ctrl+B`, `←`                                | Move cursor backward by one character     |
| `Ctrl+P`, `↑`                                | Move cursor up by one line                |
| `Ctrl+N`, `↓`                                | Move cursor down by one line              |
| `Alt+F`, `Ctrl+→`                            | Move cursor forward by word               |
| `Atl+B`, `Ctrl+←`                            | Move cursor backward by word              |
| `Alt+]`, `Alt+P`, `Ctrl+↑`                   | Move cursor up by paragraph               |
| `Alt+[`, `Alt+N`, `Ctrl+↓`                   | Move cursor down by paragraph             |
| `Ctrl+E`, `End`, `Ctrl+Alt+F`, `Ctrl+Alt+→`  | Move cursor to the end of line            |
| `Ctrl+A`, `Home`, `Ctrl+Alt+B`, `Ctrl+Alt+←` | Move cursor to the head of line           |
| `Alt+<`, `Ctrl+Alt+P`, `Ctrl+Alt+↑`          | Move cursor to top of lines               |
| `Alt+>`, `Ctrl+Alt+N`, `Ctrl+Alt+↓`          | Move cursor to bottom of lines            |
| `Ctrl+V`, `PageDown`                         | Scroll down by page                       |
| `Alt+V`, `PageUp`                            | Scroll up by page                         |

Deleting multiple characters at once saves the deleted text to yank buffer. It can be pasted with `Ctrl+Y` later.

If you don't want to use default key mappings, see the 'Advanced Usage' section.

## Basic Usage

### Create `TextArea` instance with text

`TextArea` implements `Default` trait to create an editor instance with an arbitrary text.

```rust,ignore
let mut textarea = TextArea::with_value("hello");
```

`TextArea::new()` creates an editor instance with text passed as [autosurgeon::Text](https://docs.rs/autosurgeon/latest/autosurgeon/).

```rust,ignore
let mut text = autosurgeon::Text::with_value("hello");
let mut textarea = TextArea::new(text);
```

### Get text contents from `TextArea`

`TextArea::text()` returns text as `&autosurgeon::Text`.

```rust,ignore
let text = textarea.text();
```

### Configure cursor line style

By default, `TextArea` renders the line at cursor with underline so that users can easily notice where the current line
is. To change the style of cursor line, use `TextArea::set_cursor_line_style()`. For example, the following styles the
cursor line with bold text.

```rust,ignore
use ratatui::style::{Style, Modifier};

let style = Style::default().add_modifier(Modifier::BOLD);
textarea.set_cursor_line_style(style);
```

To disable cursor line style, set the default style as follows:

```rust,ignore
use ratatui::style::{Style, Modifier};

textarea.set_cursor_line_style(Style::default());
```

### Configure tab width

The default tab width is 4. To change it, use `TextArea::set_tab_length()` method. The following sets 2 to tab width.
Typing tab key inserts 2 spaces.

```rust,ignore
textarea.set_tab_length(2);
```

## Advanced Usage

### Define your own key mappings

All editor operations are defined as public methods of `TextArea`. To move cursor, use `ratatui_mergearea::CursorMove` to
notify how to move the cursor.

| Method                                               | Operation                                       |
|------------------------------------------------------|-------------------------------------------------|
| `textarea.delete_char()`                             | Delete one character before cursor              |
| `textarea.delete_next_char()`                        | Delete one character next to cursor             |
| `textarea.insert_newline()`                          | Insert newline                                  |
| `textarea.delete_line_by_end()`                      | Delete from cursor until the end of line        |
| `textarea.delete_line_by_head()`                     | Delete from cursor until the head of line       |
| `textarea.delete_word()`                             | Delete one word before cursor                   |
| `textarea.delete_next_word()`                        | Delete one word next to cursor                  |
| `textarea.undo()`                                    | Undo                                            |
| `textarea.redo()`                                    | Redo                                            |
| `textarea.copy()`                                    | Copy selected text                              |
| `textarea.cut()`                                     | Cut selected text                               |
| `textarea.paste()`                                   | Paste yanked text                               |
| `textarea.start_selection()`                         | Start text selection                            |
| `textarea.cancel_selection()`                        | Cancel text selection                           |
| `textarea.select_all()`                              | Select entire text                              |
| `textarea.move_cursor(CursorMove::Forward)`          | Move cursor forward by one character            |
| `textarea.move_cursor(CursorMove::Back)`             | Move cursor backward by one character           |
| `textarea.move_cursor(CursorMove::Up)`               | Move cursor up by one line                      |
| `textarea.move_cursor(CursorMove::Down)`             | Move cursor down by one line                    |
| `textarea.move_cursor(CursorMove::WordForward)`      | Move cursor forward by word                     |
| `textarea.move_cursor(CursorMove::WordEnd)`          | Move cursor to next end of word                 |
| `textarea.move_cursor(CursorMove::WordBack)`         | Move cursor backward by word                    |
| `textarea.move_cursor(CursorMove::ParagraphForward)` | Move cursor up by paragraph                     |
| `textarea.move_cursor(CursorMove::ParagraphBack)`    | Move cursor down by paragraph                   |
| `textarea.move_cursor(CursorMove::End)`              | Move cursor to the end of line                  |
| `textarea.move_cursor(CursorMove::Head)`             | Move cursor to the head of line                 |
| `textarea.move_cursor(CursorMove::Top)`              | Move cursor to top of lines                     |
| `textarea.move_cursor(CursorMove::Bottom)`           | Move cursor to bottom of lines                  |
| `textarea.move_cursor(CursorMove::Jump(row, col))`   | Move cursor to (row, col) position              |
| `textarea.move_cursor(CursorMove::InViewport)`       | Move cursor to stay in the viewport             |
| `textarea.set_search_pattern(pattern)`               | Set a pattern for text search                   |
| `textarea.search_forward(match_cursor)`              | Move cursor to next match of text search        |
| `textarea.search_back(match_cursor)`                 | Move cursor to previous match of text search    |
| `textarea.scroll(Scrolling::PageDown)`               | Scroll down the viewport by page                |
| `textarea.scroll(Scrolling::PageUp)`                 | Scroll up the viewport by page                  |
| `textarea.scroll(Scrolling::HalfPageDown)`           | Scroll down the viewport by half-page           |
| `textarea.scroll(Scrolling::HalfPageUp)`             | Scroll up the viewport by half-page             |
| `textarea.scroll((row, col))`                        | Scroll down the viewport to (row, col) position |

To define your own key mappings, simply call the above methods in your code instead of `TextArea::input()` method.

See the [`vim` example](./examples/vim.rs) for working example. It implements more Vim-like key modal mappings.

If you don't want to use default key mappings, `TextArea::input()` method can be used instead of
`TextArea::input_emacs()`. The method only handles very basic operations such as inserting/deleting single characters, tabs,
newlines.

```rust,ignore
match read()?.into() {
    // Handle your own key mappings here
    // ...
    input => textarea.input(input),
}
```

