use crate::version_check::UpdateInformation;
use color_eyre::Result;
use reqwest::StatusCode;
use std::path::Path;
use tokio::fs::File;
use tokio::io::copy;

pub async fn download_patch(patch: UpdateInformation, target_dir: &Path) -> Result<()> {
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
        let response = reqwest::get(url).await?;
        assert_eq!(response.status(), StatusCode::OK);
        let content = response.bytes().await?;
        tokio::fs::create_dir_all(&target_patch_file.parent().unwrap()).await?;
        let mut file = File::create(target_patch_file).await?;
        copy(&mut content.as_ref(), &mut file).await?;
    }

    Ok(())
}
