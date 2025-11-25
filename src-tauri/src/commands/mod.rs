use std::path::PathBuf;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    config::{self, Options},
    image_ops::{build_pipeline, apply_saturation},
    output,
    preview::{self, PreviewPayload},
};

#[derive(Debug, Deserialize)]
pub struct FilePayload {
    pub name: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct ConvertResult {
    pub output_dir: String,
    pub succeeded: usize,
    pub failed: usize,
}

#[tauri::command]
pub fn generate_preview(
    payload: PreviewPayload,
) -> Result<preview::PreviewResponse, String> {
    preview::generate_preview(payload)
}

#[tauri::command]
pub fn convert_all(
    options: Options,
    files: Vec<FilePayload>,
    output_root: String,
    saturation: Option<f32>,
) -> Result<ConvertResult, String> {
    let root = PathBuf::from(output_root);
    let dir = output::create_output_dir(&root).map_err(|e| e.to_string())?;
    let pipeline = build_pipeline(&options);

    let (succeeded, failed) = files
        .par_iter()
        .enumerate()
        .map(|(idx, file)| {
            let result: Result<(), String> = (|| {
                let mut img = image::load_from_memory(&file.bytes).map_err(|e| e.to_string())?;
                img = pipeline.run(img);
                if let Some(sat) = saturation {
                    img = apply_saturation(&img, sat);
                }
                let fmt = choose_format(&file.name, &options);
                output::save_image(&dir, &file.name, idx + 1, &img, fmt)
                    .map_err(|e| e.to_string())?;
                Ok(())
            })();
            if result.is_ok() { (1usize, 0usize) } else { (0usize, 1usize) }
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    Ok(ConvertResult {
        output_dir: dir.to_string_lossy().to_string(),
        succeeded,
        failed,
    })
}

fn choose_format(name: &str, options: &Options) -> image::ImageOutputFormat {
    let lower = name.to_lowercase();
    if lower.ends_with(".webp") {
        image::ImageOutputFormat::WebP(options.webp_quality)
    } else if lower.ends_with(".png") {
        image::ImageOutputFormat::Png
    } else {
        image::ImageOutputFormat::Jpeg(options.jpeg_quality)
    }
}

#[tauri::command]
pub fn save_preset(name: String, options: Options) -> Result<(), String> {
    config::save_preset(&name, &options).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_preset(name: String) -> Result<Options, String> {
    config::load_preset(&name).map_err(|e| e.to_string())
}
