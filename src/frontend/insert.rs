use tokio::sync::mpsc;
use url::Url;

use crate::backend::download_job::{DownloadJob, Status};

use super::{app_event::AppEvent, event::EventHandler};

#[derive(Debug)]
pub struct Insert {
    pub url: String,
    pub character_index: usize,
}

impl Insert {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            character_index: 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.url.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.url
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.url.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.url.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.url.chars().skip(current_index);
            self.url = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.url.chars().count())
    }

    pub fn clean(&mut self) {
        self.url.clear();
        self.character_index = 0;
    }

    pub fn submit_message(
        &mut self,
        events: &mut EventHandler,
        sender: mpsc::UnboundedSender<DownloadJob>,
    ) {
        self.url = self.url.trim().to_string();
        if self.url.is_empty() {
            self.clean();
            return;
        }

        let Ok(url) = Url::parse(&self.url) else {
            let mut job = DownloadJob::new("", "Failed to parse URL");
            job.status = Status::Failed;
            events.send(AppEvent::Job(job));
            self.clean();
            return;
        };

        let Some(file_path) = url.path_segments() else {
            let mut job = DownloadJob::new("", "Failed to get file name");
            job.status = Status::Failed;
            events.send(AppEvent::Job(job));
            self.clean();
            return;
        };
        let file_name = file_path.last().unwrap_or("Failed to get file name");
        if file_name.is_empty() {
            let mut job = DownloadJob::new(url.as_str(), "Failed to get file name");
            job.status = Status::Failed;
            events.send(AppEvent::Job(job));
            self.clean();
            return;
        }

        let job = DownloadJob::new(url.as_str(), file_name);
        events.send(AppEvent::Job(job.clone()));
        let _ = sender.send(job);
        self.clean();
    }
}
