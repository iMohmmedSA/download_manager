use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType},
};

use super::View;

impl View {
    pub fn header(frame: &mut Frame, area: Rect) {
        let title = Block::bordered()
            .title(" Download Manager ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        frame.render_widget(title, area);
    }
}
