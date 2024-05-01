use crate::download::download_patch;
use crate::version_check::start_checker;
use color_eyre::Result;
use std::path::Path;
use std::time::Duration;

mod download;
mod packets;
mod version_check;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut receiver = start_checker(589, Duration::from_secs(60 * 60 * 5));
    let target_dir = Path::new("./patches");
    while let Some(patch) = receiver.recv().await {
        let err = download_patch(patch, target_dir).await;
        if let Err(e) = err {
            println!("{e}");
        }
    }
    Ok(())
}
