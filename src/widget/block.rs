use tui::text::Span;
use tui::widgets::{Block, Borders};

use crate::theme::style;
use crate::THEME;

pub fn new(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        // border color updated from secondary to gray
        .border_style(style().fg(THEME.gray()))
        .title(Span::styled(title, style().fg(THEME.text_normal())))
}
