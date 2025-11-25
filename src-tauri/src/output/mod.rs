use std::{fs, path::PathBuf};

use chrono::Local;
use image::{DynamicImage, ImageOutputFormat};

pub fn create_output_dir(base: &PathBuf) -> std::io::Result<PathBuf> {
    let dir = base.join(format!("output_{}", Local::now().format("%Y%m%d_%H%M%S")));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn save_image(
    dir: &PathBuf,
    original_name: &str,
    seq: usize,
    img: &DynamicImage,
    fmt: ImageOutputFormat,
) -> std::io::Result<PathBuf> {
    let ext = match fmt {
        ImageOutputFormat::Jpeg(_) => "jpg",
        ImageOutputFormat::Png => "png",
        ImageOutputFormat::WebP(_) => "webp",
        _ => "img",
    };
    let filename = format!("{}_mod_{:03}.{}", original_name, seq, ext);
    let path = dir.join(filename);
    let mut buf = Vec::new();
    img.write_to(&mut buf, fmt.clone()).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(&path, buf)?;
    Ok(path)
}
