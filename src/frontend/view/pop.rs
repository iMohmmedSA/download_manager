use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Cell, Clear, HighlightSpacing, Row, Table},
};

use crate::app::App;

use super::View;

impl View {
    pub fn pop_size(area: Rect) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(60)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub fn pop(app: &mut App, frame: &mut Frame) {
        let block = Block::bordered()
            .title(" Dev list of files ")
            .border_type(BorderType::Rounded);

        let rows = app.dev_table.items.iter();
        let widths = [
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ];
        let highlight_style = Style::default().bg(Color::DarkGray);
        let header_style = Style::default().fg(Color::White).bg(Color::Blue);
        let header = ["Name", "Size", "URL"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let table = Table::new(rows, widths)
            .block(block)
            .header(header)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("-> ")
            .row_highlight_style(highlight_style);

        let area = Self::pop_size(frame.area());
        frame.render_widget(Clear, area);
        frame.render_stateful_widget(table, area, &mut app.dev_table.state);
    }
}
