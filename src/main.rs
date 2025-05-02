use backend::{download::Download, download_job::DownloadJob};
use tokio::sync::mpsc;

use crate::app::App;

mod app;
mod backend;
mod frontend;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let (sender, receiver) = mpsc::unbounded_channel::<DownloadJob>();

    let app = App::new(sender);
    Download::new(app.events.sender.clone(), receiver).run();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app.run(terminal).await;
    ratatui::restore();
    result
}
