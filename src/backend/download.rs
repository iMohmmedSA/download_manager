use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use directories::UserDirs;
use futures::future::join_all;
use reqwest::{
    Client, Response,
    header::{ACCEPT_RANGES, CONTENT_LENGTH},
};
use tokio::io::AsyncWriteExt;
use tokio::{fs::OpenOptions, sync::mpsc};
use tokio::{io::AsyncSeekExt, time::Instant};
use tokio_stream::StreamExt;

use crate::frontend::{app_event::AppEvent, event::Event};

use super::download_job::{DownloadJob, DownloadStrategy, Status};

const MAX_ATTEMPTS: u8 = 5;
const NUM_PARTS: u64 = 4;

pub struct Download {
    app_sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<DownloadJob>,
}

impl Download {
    pub fn new(
        app_sender: mpsc::UnboundedSender<Event>,
        receiver: mpsc::UnboundedReceiver<DownloadJob>,
    ) -> Self {
        Self {
            app_sender,
            receiver,
        }
    }

    pub fn run(self) {
        let mut receiver = self.receiver;
        tokio::spawn(async move {
            while let Some(mut job) = receiver.recv().await {
                let app_sender = self.app_sender.clone();
                tokio::spawn(async move { Self::handle_job(app_sender, &mut job).await });
            }
        });
    }

    async fn handle_job(sender: mpsc::UnboundedSender<Event>, mut job: &mut DownloadJob) {
        Self::metadata(sender.clone(), &mut job).await;
        match job.strategy {
            DownloadStrategy::Parallel => Self::download_parallel(sender.clone(), &mut job).await,
            DownloadStrategy::Sequential => {
                Self::download_sequential(sender.clone(), &mut job).await
            }
            DownloadStrategy::Unknown => {
                // TODO: Implement handling for unknown download strategy
            }
        }
    }

    async fn metadata(sender: mpsc::UnboundedSender<Event>, job: &mut DownloadJob) {
        let mut attempts = 0;
        let mut delay = Duration::from_secs(5);

        while attempts < MAX_ATTEMPTS {
            let client = Client::new();
            match client.head(&job.url).send().await {
                Ok(response) if response.status().is_success() => {
                    Self::parse_header(response, sender.clone(), job).await;
                    break;
                }
                Ok(_) => {
                    attempts += 1;
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }
                Err(_) => {
                    attempts += 1;
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                    // TODO: Handle error
                }
            }
        }
    }

    async fn parse_header(
        response: Response,
        sender: mpsc::UnboundedSender<Event>,
        job: &mut DownloadJob,
    ) {
        if let Some(len_value) = response.headers().get(CONTENT_LENGTH) {
            if let Ok(len_str) = len_value.to_str() {
                if let Ok(length) = len_str.parse::<u64>() {
                    job.size = length;
                    sender
                        .send(Event::App(AppEvent::JobSize(job.id, length)))
                        .ok();
                }
            }
        }

        let support_range = response
            .headers()
            .get(ACCEPT_RANGES)
            .map(|b| {
                if b == "bytes" {
                    DownloadStrategy::Parallel
                } else {
                    DownloadStrategy::Sequential
                }
            })
            .unwrap_or(DownloadStrategy::Sequential);

        let _ = sender.send(Event::App(AppEvent::JobStrategy(
            job.id,
            support_range.clone(),
        )));

        job.strategy = support_range;
        return;
    }

    async fn download_parallel(sender: mpsc::UnboundedSender<Event>, job: &mut DownloadJob) {
        let part_size = job.size / NUM_PARTS;
        let client = Client::new();
        let Some(path) = Self::get_download_path(&job.filename) else {
            sender
                .send(Event::App(AppEvent::JobStatus(job.id, Status::Failed)))
                .ok();
            return;
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path.clone())
            .await
            .expect("Failed to create file");
        file.set_len(job.size)
            .await
            .expect("Failed to preallocate file");

        drop(file);

        sender
            .send(Event::App(AppEvent::JobStatus(job.id, Status::InProgress)))
            .ok();

        let total_downloaded = Arc::new(AtomicU64::new(0));
        let mut tasks = vec![];

        {
            let total = job.size;
            let total_downloaded = total_downloaded.clone();
            let id = job.id;
            let sender = sender.clone();

            let start_time = Instant::now();
            let monitor = tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    let downloaded = total_downloaded.load(Ordering::Relaxed);
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 {
                        downloaded as f64 / elapsed
                    } else {
                        0.0
                    };

                    let eta = if speed > 0.0 {
                        (total - downloaded) as f64 / speed
                    } else {
                        0.0
                    };

                    let _ = sender.send(Event::App(AppEvent::JobProgress(
                        id, downloaded, speed, eta as u64,
                    )));
                    if downloaded >= total {
                        break;
                    }
                }
            });

            tasks.push(monitor);
        }

        for i in 0..NUM_PARTS {
            let start = i * part_size;
            let end = if i == NUM_PARTS - 1 {
                job.size - 1
            } else {
                (i + 1) * part_size - 1
            };

            let url = job.url.clone();
            let path = path.clone();
            let client = client.clone();
            let progress = total_downloaded.clone();
            let id = job.id;
            let sender = sender.clone();

            let task = tokio::spawn(async move {
                let range_header = format!("bytes={}-{}", start, end);
                let Ok(res) = client.get(&url).header("Range", range_header).send().await else {
                    sender
                        .send(Event::App(AppEvent::JobStatus(id, Status::Failed)))
                        .ok();
                    return;
                };

                let mut chunk = res.bytes_stream();
                let Ok(mut file) = OpenOptions::new().write(true).open(&path).await else {
                    sender
                        .send(Event::App(AppEvent::JobStatus(id, Status::Failed)))
                        .ok();
                    return;
                };

                let Ok(_) = file.seek(std::io::SeekFrom::Start(start)).await else {
                    sender
                        .send(Event::App(AppEvent::JobStatus(id, Status::Failed)))
                        .ok();
                    return;
                };

                while let Some(Ok(bytes)) = chunk.next().await {
                    file.write_all(&bytes).await.expect("Failed to write chunk");
                    progress.fetch_add(bytes.len() as u64, Ordering::Relaxed);
                }
            });

            tasks.push(task);
        }

        let _ = join_all(tasks).await;
        sender
            .send(Event::App(AppEvent::JobStatus(job.id, Status::Completed)))
            .ok();
    }

    async fn download_sequential(sender: mpsc::UnboundedSender<Event>, job: &mut DownloadJob) {
        sender
            .send(Event::App(AppEvent::JobStatus(job.id, Status::InProgress)))
            .ok();

        let client = Client::new();
        let Some(path) = Self::get_download_path(&job.filename) else {
            sender
                .send(Event::App(AppEvent::JobStatus(job.id, Status::Failed)))
                .ok();
            return;
        };

        let response = match client.get(&job.url).send().await {
            Ok(resp) if resp.status().is_success() => resp,
            _ => {
                sender
                    .send(Event::App(AppEvent::JobStatus(job.id, Status::Failed)))
                    .ok();
                return;
            }
        };

        let mut stream = response.bytes_stream();

        let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .await
        else {
            sender
                .send(Event::App(AppEvent::JobStatus(job.id, Status::Failed)))
                .ok();
            return;
        };

        let total_downloaded = Arc::new(AtomicU64::new(0));
        let downloaded_clone = total_downloaded.clone();
        let sender_clone = sender.clone();
        let job_id = job.id;

        let start_time = Instant::now();

        let monitor = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;

                let downloaded = downloaded_clone.load(Ordering::Relaxed);
                let elapsed = start_time.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    downloaded as f64 / elapsed
                } else {
                    0.0
                };

                let _ = sender_clone.send(Event::App(AppEvent::JobProgress(
                    job_id, downloaded, speed, 0,
                )));
            }
        });

        while let Some(Ok(chunk)) = stream.next().await {
            if let Err(_) = file.write_all(&chunk).await {
                monitor.abort();
                sender
                    .send(Event::App(AppEvent::JobStatus(job_id, Status::Failed)))
                    .ok();
                return;
            }
            total_downloaded.fetch_add(chunk.len() as u64, Ordering::Relaxed);
        }

        let _ = monitor.abort();

        sender
            .send(Event::App(AppEvent::JobStatus(job.id, Status::Completed)))
            .ok();
    }

    fn get_download_path(filename: &str) -> Option<PathBuf> {
        let user_dirs = UserDirs::new()?;
        let downloads_dir = user_dirs.download_dir()?;

        Some(downloads_dir.join(filename))
    }
}
