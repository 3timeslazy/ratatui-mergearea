#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod cursor;
mod highlight;
mod history;
mod history_v2;
mod input;
mod scroll;
#[cfg(feature = "search")]
mod search;
mod textarea;
mod util;
mod widget;
mod word;

#[allow(clippy::single_component_path_imports)]
use ratatui;

#[cfg(feature = "crossterm")]
#[allow(clippy::single_component_path_imports)]
use crossterm;

#[cfg(feature = "termion")]
#[allow(clippy::single_component_path_imports)]
use termion;

pub use cursor::CursorMove;
pub use input::{Input, Key};
pub use scroll::Scrolling;
pub use textarea::MergeArea;
