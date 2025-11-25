use image::{DynamicImage, ImageOutputFormat};
use serde::{Deserialize, Serialize};

use crate::{config::Options, image_ops::{build_pipeline, apply_saturation}};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewPayload {
    pub options: Options,
    pub image_bytes: Vec<u8>,
    pub saturation: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    pub bytes: Vec<u8>,
}

pub fn generate_preview(payload: PreviewPayload) -> Result<PreviewResponse, String> {
    let img = image::load_from_memory(&payload.image_bytes).map_err(|e| e.to_string())?;
    let thumb = img.thumbnail(512, 512);
    let pipeline = build_pipeline(&payload.options);
    let mut out = pipeline.run(thumb);
    if let Some(sat) = payload.saturation {
        out = apply_saturation(&out, sat);
    }
    let mut buf = Vec::new();
    out.write_to(&mut buf, ImageOutputFormat::Png)
        .map_err(|e| e.to_string())?;
    Ok(PreviewResponse { bytes: buf })
}
