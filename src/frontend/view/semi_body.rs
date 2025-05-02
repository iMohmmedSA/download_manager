use ratatui::{
    Frame,
    layout::{Alignment, Position, Rect},
    style::Stylize,
    symbols::{
        border,
        line::{ROUNDED_BOTTOM_LEFT, ROUNDED_BOTTOM_RIGHT},
    },
    widgets::{Block, BorderType, Paragraph},
};

use crate::{app::App, frontend::app_mode::AppMode};

use super::View;

impl View {
    pub fn semi_body(app: &App, frame: &mut Frame, area: Rect) {
        match app.mode {
            AppMode::Normal => {
                View::normal(frame, area);
            }
            AppMode::Insert => {
                View::insert_mode(app, frame, area);
            }
            AppMode::Table => {
                View::table_mode(frame, area);
            }
        }
    }

    fn normal(frame: &mut Frame, area: Rect) {
        let mut border = border::ROUNDED;
        border.top_left = ROUNDED_BOTTOM_LEFT;
        border.top_right = ROUNDED_BOTTOM_RIGHT;

        let title = Block::bordered()
            .border_set(border)
            .title_bottom(" Normal Mode ")
            .title_alignment(Alignment::Center);

        frame.render_widget(title, area);
    }

    fn insert_mode(app: &App, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(app.insert.url.as_str())
            .block(
                Block::bordered()
                    .title(" Enter file download link. ")
                    .border_type(BorderType::Rounded),
            )
            .cyan();

        frame.render_widget(input, area);
        frame.set_cursor_position(Position::new(
            area.x + app.insert.character_index as u16 + 1,
            area.y + 1,
        ));
    }

    fn table_mode(frame: &mut Frame, area: Rect) {
        let mut border = border::ROUNDED;
        border.top_left = ROUNDED_BOTTOM_LEFT;
        border.top_right = ROUNDED_BOTTOM_RIGHT;

        let title = Block::bordered()
            .border_set(border)
            .title_bottom(" Table Mode ")
            .title_alignment(Alignment::Center);

        frame.render_widget(title, area);
    }
}
