use std::{
    fs::File,
    io::BufWriter,
    path::PathBuf,
};

use chrono::Local;
use image::{codecs::webp::WebPEncoder, ColorType, DynamicImage, ImageEncoder, ImageOutputFormat};

pub enum SaveFormat {
    Png,
    Jpeg(u8),
    Webp(u8),
}

pub fn create_output_dir(base: &PathBuf) -> std::io::Result<PathBuf> {
    let dir = base.join(format!("output_{}", Local::now().format("%Y%m%d_%H%M%S")));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn save_image(
    dir: &PathBuf,
    original_name: &str,
    seq: usize,
    img: &DynamicImage,
    fmt: SaveFormat,
) -> std::io::Result<PathBuf> {
    let path = {
        let ext = match fmt {
            SaveFormat::Png => "png",
            SaveFormat::Jpeg(_) => "jpg",
            SaveFormat::Webp(_) => "webp",
        };
        let filename = format!("{}_mod_{:03}.{}", original_name, seq, ext);
        dir.join(filename)
    };

    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    match fmt {
        SaveFormat::Png => {
            img.write_to(&mut writer, ImageOutputFormat::Png)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }
        SaveFormat::Jpeg(quality) => {
            img.write_to(&mut writer, ImageOutputFormat::Jpeg(quality))
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }
        SaveFormat::Webp(_quality) => {
            let rgba = img.to_rgba8();
            let encoder = WebPEncoder::new_lossless(&mut writer);
            encoder
                .write_image(rgba.as_raw(), img.width(), img.height(), ColorType::Rgba8)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }
    }

    Ok(path)
}
