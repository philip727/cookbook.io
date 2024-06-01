use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;
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
