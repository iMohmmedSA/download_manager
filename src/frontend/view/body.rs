use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Cell, HighlightSpacing, Row, Table},
};

use crate::app::App;

use super::View;

impl View {
    pub fn body(app: &mut App, frame: &mut Frame, area: Rect) {
        let title = Block::bordered()
            .title(" List ")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let rows = app.table.items.iter();
        let widths = [
            Constraint::Fill(1),
            Constraint::Length(13),
            Constraint::Length(58),
            Constraint::Length(12),
            Constraint::Fill(1),
        ];

        let highlight_style = Style::default().bg(Color::DarkGray);
        let header_style = Style::default().fg(Color::White).bg(Color::Blue);
        let header = ["Name", "Status", "Progress", "Strategy", "URL"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let table = Table::new(rows, widths)
            .block(title)
            .header(header)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("-> ")
            .row_highlight_style(highlight_style);

        frame.render_stateful_widget(table, area, &mut app.table.state);
    }
}
