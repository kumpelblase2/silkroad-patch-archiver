use crate::version_check::UpdateInformation;
use bytes::Bytes;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::path::Path;
use tokio::fs::File;
use tokio::io::copy;

pub async fn download_patch(patch: &UpdateInformation, target_dir: &Path) -> Result<()> {
    let patch_folder = target_dir.join(patch.new_version.to_string());
    if !&patch_folder.exists() {
        tokio::fs::create_dir(&patch_folder).await?;
    }
    let server = &patch.http_server;
    for file in patch.files.iter() {
        let non_windows_path = file.file_path.replace("\\", "/");
        let filename_with_path = Path::new(&non_windows_path).join(&file.filename);
        let target_patch_file = patch_folder.join(&filename_with_path);
        if target_patch_file.exists() {
            continue;
        }
        let url = format!("http://{server}/{}", filename_with_path.to_str().unwrap());
        println!("{}", url);
        let content = download_file(url).await;
        match content {
            Ok(content) => {
                tokio::fs::create_dir_all(&target_patch_file.parent().unwrap()).await?;
                let mut file = File::create(target_patch_file).await?;
                copy(&mut content.as_ref(), &mut file).await?;
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }

    Ok(())
}

async fn download_file(url: String) -> Result<Bytes> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(eyre!("Didn't get an OK"));
    }
    let content = response.bytes().await?;
    Ok(content)
}
