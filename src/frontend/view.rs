pub mod body;
pub mod footer;
pub mod header;
pub mod pop;
pub mod semi_body;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::app::App;

use super::app_mode::AppMode;

pub struct View;

impl View {
    pub fn build(app: &mut App, frame: &mut Frame) {
        let [header, body, semi_body, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(if app.mode == AppMode::Insert { 3 } else { 1 }),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        View::header(frame, header);
        View::body(app, frame, body);
        View::semi_body(app, frame, semi_body);
        View::footer(app, frame, footer);

        if app.show_popup {
            Self::pop(app, frame);
        }
    }
}
