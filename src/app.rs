use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::{
    backend::{dev::DevList, download_job::DownloadJob},
    frontend::{
        app_event::AppEvent,
        app_mode::AppMode,
        download_list_state::ListState,
        event::{Event, EventHandler},
        insert::Insert,
        key_events::KeyEvent,
        view::View,
    },
};

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,

    pub events: EventHandler,
    pub download_sender: mpsc::UnboundedSender<DownloadJob>,
    pub insert: Insert,

    pub mode: AppMode,

    // DEV
    pub show_popup: bool,
    pub dev_table: ListState<DevList>,

    pub table: ListState<DownloadJob>,
}

impl App {
    pub fn new(sender: mpsc::UnboundedSender<DownloadJob>) -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            download_sender: sender,
            insert: Insert::new(),
            mode: AppMode::Normal,
            show_popup: false,
            dev_table: DevList::list(),
            table: ListState::new(),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| View::build(&mut self, frame))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        KeyEvent::handle_key_events(&mut self, key_event)?
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Job(job) => {
                        self.table.items.push(job);
                    }
                    AppEvent::JobSize(id, length) => {
                        if let Some(job) = self.table.items.iter_mut().find(|job| job.id == id) {
                            job.size = length;
                        }
                    }
                    AppEvent::JobStrategy(id, strategy) => {
                        if let Some(job) = self.table.items.iter_mut().find(|job| job.id == id) {
                            job.strategy = strategy;
                        }
                    }
                    AppEvent::JobProgress(id, progress, speed, eta) => {
                        if let Some(job) = self.table.items.iter_mut().find(|job| job.id == id) {
                            job.progress = progress;
                            job.speed = speed;
                            job.eta = eta;
                        }
                    }
                    AppEvent::JobStatus(id, status) => {
                        if let Some(job) = self.table.items.iter_mut().find(|job| job.id == id) {
                            job.status = status;
                        }
                    }
                    AppEvent::MoveUp => {
                        self.table.move_up();
                    }
                    AppEvent::MoveDown => {
                        self.table.move_down();
                    }
                    AppEvent::EscInsert => {
                        self.insert.clean();
                        self.mode = AppMode::Normal;
                    }
                    AppEvent::Backspace => self.insert.delete_char(),
                    AppEvent::EnterChar(char) => self.insert.enter_char(char),
                    AppEvent::MoveCursorLeft => self.insert.move_cursor_left(),
                    AppEvent::MoveCursorRight => self.insert.move_cursor_right(),
                    AppEvent::EnterInsert => self
                        .insert
                        .submit_message(&mut self.events, self.download_sender.clone()),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
