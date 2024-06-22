use crate::config::load_config;
use crate::download::download_patch;
use crate::version_check::start_checker;
use color_eyre::Result;
use config::save_config;
use std::path::Path;
use std::time::Duration;

mod config;
mod download;
mod packets;
mod version_check;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut config = load_config();
    let mut receiver = start_checker(config.current_version, Duration::from_secs(60 * 60 * 5));
    let target_dir = Path::new("./patches");

    loop {
        tokio::select! {
            info = receiver.recv() => {
                match info {
                    Some(patch) => {
                        let err = download_patch(&patch, target_dir).await;
                        if let Err(e) = err {
                            println!("{e}");
                        } else {
                            config.current_version = patch.new_version;
                            save_config(&config).expect("Should be able to save config.");
                        }
                    },
                    None => break,
                }
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    Ok(())
}
