use uuid::Uuid;

use crate::backend::download_job::{DownloadJob, DownloadStrategy, Status};

/// Application events.
///
/// You can extend this enum with your own custom events.
#[derive(Clone, Debug)]
pub enum AppEvent {
    /// Job
    Job(DownloadJob),
    JobSize(Uuid, u64),
    JobStrategy(Uuid, DownloadStrategy),
    JobProgress(Uuid, u64, f64, u64),
    JobStatus(Uuid, Status),

    /// InsertMode
    EscInsert,
    EnterChar(char),
    Backspace,
    MoveCursorLeft,
    MoveCursorRight,
    EnterInsert,

    // Navigate
    MoveUp,
    MoveDown,

    /// Quit the application.
    Quit,
}
