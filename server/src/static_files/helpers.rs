use std::{
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::Multipart;
use actix_web::web;
use anyhow::anyhow;
use futures::{StreamExt, TryStreamExt};

pub async fn save_file(
    mut payload: Multipart,
    file_path: &'static str,
) -> Result<(), anyhow::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        //let content_type = field.content_disposition();

        let mut f = web::block(move || std::fs::File::create(file_path)).await??;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
    }

    Ok(())
}

pub fn rename_temp_file(
    temp_file: TempFile,
    new_path: &str,
    new_file_name: &str,
) -> Result<String, anyhow::Error> {
    let temp_file_path = temp_file.file.path();
    let file_name = temp_file
        .file_name
        .ok_or(anyhow!("No file name in content-disposition header"))?;

    let file_ext = Path::new(&file_name)
        .extension()
        .ok_or(anyhow!("File has no extension"))?
        .to_string_lossy();

    let file_name: String = new_file_name.to_owned() + "." + &file_ext;
    let mut file_path = PathBuf::from_str(new_path)?;
    file_path.push(&sanitize_filename::sanitize(&file_name));
    std::fs::rename(temp_file_path, file_path)?;

    Ok(file_name)
}
