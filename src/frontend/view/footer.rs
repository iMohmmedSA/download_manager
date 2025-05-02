use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
};

use crate::{app::App, frontend::app_mode::AppMode};

use super::View;

impl View {
    pub fn footer(app: &App, frame: &mut Frame, area: Rect) {
        let mut mode_instruction = match app.mode {
            AppMode::Normal => {
                vec![
                    "Press ".into(),
                    "[I]".bold().cyan(),
                    " to enter a URL to download a file. Press ".into(),
                    "[T]".bold().cyan(),
                    " to navigate.".into(),
                ]
            }
            AppMode::Insert => {
                vec![
                    "Press ".into(),
                    "[Esc]".bold().cyan(),
                    " to return back.".into(),
                ]
            }
            AppMode::Table => {
                vec![
                    "Press ".into(),
                    "[Esc]".bold().cyan(),
                    " to return back. ".into(),
                    "[↑]".bold().cyan(),
                    " to move up.".into(),
                    "[↓]".bold().cyan(),
                    " to move down.".into(),
                ]
            }
        };

        mode_instruction.append(&mut vec![
            " Press ".into(),
            "[Ctrl+C]".bold().cyan(),
            " to exit.".into(),
        ]);

        let footer = Text::from(Line::from(mode_instruction)).centered();
        frame.render_widget(footer, area);
    }
}
