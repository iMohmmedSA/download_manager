use crossterm::event::{self, KeyCode, KeyModifiers};

use crate::app::App;

use super::{app_event::AppEvent, app_mode::AppMode, event::EventHandler};

pub struct KeyEvent;

impl KeyEvent {
    pub fn handle_key_events(app: &mut App, key_event: event::KeyEvent) -> color_eyre::Result<()> {
        if app.show_popup {
            KeyEvent::handle_popup(app, key_event);
            return Ok(());
        }

        match app.mode {
            AppMode::Normal => {
                KeyEvent::handle_normal(app, key_event);
            }
            AppMode::Insert => {
                KeyEvent::handle_input_mode(&mut app.events, key_event);
            }
            AppMode::Table => {
                KeyEvent::handle_table_mode(&mut app.events, key_event);
            }
        }
        Ok(())
    }

    fn handle_popup(app: &mut App, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                app.events.send(AppEvent::Quit)
            }
            KeyCode::Esc | KeyCode::Char('d' | 'D') => app.show_popup = false,
            KeyCode::Up => {
                app.dev_table.move_up();
            }
            KeyCode::Down => {
                app.dev_table.move_down();
            }
            KeyCode::Enter => {
                let Some(index) = app.dev_table.state.selected() else {
                    return;
                };
                let row = app.dev_table.items[index].clone();
                if row.url.is_empty() {
                    return;
                }
                app.insert.url = row.url;
                app.insert
                    .submit_message(&mut app.events, app.download_sender.clone());
            }
            _ => {}
        }
    }

    fn handle_normal(app: &mut App, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                app.events.send(AppEvent::Quit)
            }
            KeyCode::Char('i' | 'I') => app.mode = AppMode::Insert,
            KeyCode::Char('t' | 'T') => app.mode = AppMode::Table,
            KeyCode::Char('d' | 'D') => app.show_popup = true,
            _ => {}
        }
    }

    fn handle_input_mode(events: &mut EventHandler, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                events.send(AppEvent::Quit)
            }
            KeyCode::Esc => events.send(AppEvent::EscInsert),
            KeyCode::Backspace => events.send(AppEvent::Backspace),
            KeyCode::Char(char) => events.send(AppEvent::EnterChar(char)),
            KeyCode::Left => events.send(AppEvent::MoveCursorLeft),
            KeyCode::Right => events.send(AppEvent::MoveCursorRight),
            KeyCode::Enter => events.send(AppEvent::EnterInsert),
            _ => {}
        }
    }

    fn handle_table_mode(events: &mut EventHandler, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                events.send(AppEvent::Quit)
            }
            KeyCode::Esc => events.send(AppEvent::EscInsert),
            KeyCode::Up => events.send(AppEvent::MoveUp),
            KeyCode::Down => events.send(AppEvent::MoveDown),
            _ => {}
        }
    }
}
