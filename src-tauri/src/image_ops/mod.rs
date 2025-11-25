use std::f32::consts::PI;

use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use rand::Rng;

use crate::config::Options;

pub trait ImageTransform {
    fn apply(&self, img: DynamicImage) -> DynamicImage;
}

pub struct ResizeTransform {
    pub min_scale: f32,
    pub max_scale: f32,
}

impl ImageTransform for ResizeTransform {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        let scale = rand::thread_rng().gen_range(self.min_scale..=self.max_scale);
        let (w, h) = img.dimensions();
        let new_w = ((w as f32) * scale).round() as u32;
        let new_h = ((h as f32) * scale).round() as u32;
        img.resize(new_w.max(1), new_h.max(1), imageops::FilterType::Lanczos3)
    }
}

pub struct RotateTransform {
    pub max_deg: f32,
}

impl ImageTransform for RotateTransform {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        if self.max_deg <= 0.0 {
            return img;
        }
        let deg = rand::thread_rng().gen_range(-self.max_deg..=self.max_deg);
        let rad = deg * PI / 180.0;
        let rgba = img.to_rgba8();
        let rotated = rotate_about_center(&rgba, rad, Interpolation::Nearest, Rgba([0, 0, 0, 0]));
        DynamicImage::ImageRgba8(rotated)
    }
}

pub struct BrightnessContrastTransform {
    pub brightness_range: f32,
    pub contrast_range: f32,
}

impl ImageTransform for BrightnessContrastTransform {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        let mut rng = rand::thread_rng();
        let b_delta = rng.gen_range(-self.brightness_range..=self.brightness_range) as i32;
        let c_delta = rng.gen_range(-self.contrast_range..=self.contrast_range);
        let adjusted = imageops::colorops::brighten(&img, b_delta);
        imageops::colorops::contrast(&adjusted, c_delta as f32)
    }
}

pub struct NoiseTransform {
    pub sigma: f32,
}

impl ImageTransform for NoiseTransform {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        if self.sigma <= 0.0 {
            return img;
        }
        let mut rng = rand::thread_rng();
        let mut buf = img.to_rgba8();
        for pixel in buf.pixels_mut() {
            let channels = pixel.channels_mut();
            for c in channels.iter_mut().take(3) {
                let noise = rng.gen_range(-self.sigma..=self.sigma);
                let v = (*c as f32 + noise).clamp(0.0, 255.0);
                *c = v as u8;
            }
        }
        DynamicImage::ImageRgba8(buf)
    }
}

pub struct StripExif;

impl ImageTransform for StripExif {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img
    }
}

pub struct Pipeline {
    stages: Vec<Box<dyn ImageTransform + Send + Sync>>,
}

impl Pipeline {
    pub fn new(stages: Vec<Box<dyn ImageTransform + Send + Sync>>) -> Self {
        Self { stages }
    }

    pub fn run(&self, img: DynamicImage) -> DynamicImage {
        self.stages.iter().fold(img, |acc, stage| stage.apply(acc))
    }
}

pub fn build_pipeline(options: &Options) -> Pipeline {
    let mut stages: Vec<Box<dyn ImageTransform + Send + Sync>> = Vec::new();
    stages.push(Box::new(ResizeTransform {
        min_scale: options.resize_min,
        max_scale: options.resize_max,
    }));
    stages.push(Box::new(RotateTransform {
        max_deg: options.rotate_max_deg,
    }));
    stages.push(Box::new(BrightnessContrastTransform {
        brightness_range: options.brightness_range,
        contrast_range: options.contrast_range,
    }));
    stages.push(Box::new(NoiseTransform {
        sigma: options.noise_sigma,
    }));
    if options.strip_exif {
        stages.push(Box::new(StripExif));
    }
    Pipeline::new(stages)
}

pub fn apply_saturation(img: &DynamicImage, amount: f32) -> DynamicImage {
    let rgba = img.to_rgba8();
    let saturated: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(rgba.width(), rgba.height(), |x, y| {
        let mut p = rgba.get_pixel(x, y);
        let channels = p.channels_mut();
        let r = channels[0] as f32;
        let g = channels[1] as f32;
        let b = channels[2] as f32;
        let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        channels[0] = ((r - luma) * amount + luma).clamp(0.0, 255.0) as u8;
        channels[1] = ((g - luma) * amount + luma).clamp(0.0, 255.0) as u8;
        channels[2] = ((b - luma) * amount + luma).clamp(0.0, 255.0) as u8;
        p
    });
    DynamicImage::ImageRgba8(saturated)
}
