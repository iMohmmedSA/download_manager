use ratatui::widgets::Row;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStrategy {
    Unknown,
    Sequential,
    Parallel,
}

#[derive(Debug, Clone)]
pub struct DownloadJob {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
    pub status: Status,
    pub strategy: DownloadStrategy,
    pub progress: u64,
    pub size: u64,
    pub speed: f64,
    pub eta: u64,
}

impl DownloadJob {
    pub fn new(url: &str, filename: &str) -> Self {
        DownloadJob {
            id: Uuid::new_v4(),
            filename: filename.to_string(),
            url: url.to_string(),
            status: Status::Pending,
            strategy: DownloadStrategy::Unknown,
            progress: 0,
            size: 1,
            speed: 0.0,
            eta: 0,
        }
    }

    pub fn eta(&self) -> String {
        if self.eta == 0 {
            return "N/A".to_string();
        }

        let hours = self.eta / 3600;
        let minutes = (self.eta % 3600) / 60;
        let secs = self.eta % 60;

        match (hours, minutes) {
            (0, 0) => format!("{:02}s", secs),
            (0, _) => format!("{}m {:02}s", minutes, secs),
            _ => format!("{}h {}m {:02}s", hours, minutes, secs),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}

impl std::fmt::Display for DownloadStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadStrategy::Unknown => write!(f, "Unknown"),
            DownloadStrategy::Sequential => write!(f, "Sequential"),
            DownloadStrategy::Parallel => write!(f, "Parallel"),
        }
    }
}

impl From<&DownloadJob> for Row<'_> {
    fn from(job: &DownloadJob) -> Self {
        let job = job.clone();
        let size = if job.size == 0 { 1 } else { job.size };
        let done_format = bytesize::ByteSize(job.progress);
        let size_format = bytesize::ByteSize(job.size);
        let job_eta = match job.status {
            Status::InProgress if job.strategy == DownloadStrategy::Parallel => {
                format!(" - ETA: {}", job.eta())
            }
            _ => "".to_string(),
        };
        let job_speed = match job.status {
            Status::InProgress => {
                format!("- {}/s{}", bytesize::ByteSize(job.speed as u64), job_eta)
            }
            _ => "".to_string(),
        };

        let progress = match job.strategy {
            DownloadStrategy::Parallel => format!(
                "[{:>5.1}% {}/{} {}]",
                (job.progress as f64 / size as f64) * 100.0,
                done_format,
                size_format,
                job_speed
            ),
            DownloadStrategy::Sequential => {
                format!("[{} {}]", done_format.to_string(), job_speed)
            }
            DownloadStrategy::Unknown => "".to_string(),
        };

        Row::new(vec![
            job.filename,
            job.status.to_string(),
            progress,
            job.strategy.to_string(),
            job.url,
        ])
    }
}
