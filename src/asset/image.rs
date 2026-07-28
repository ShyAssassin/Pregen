use std::path::PathBuf;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    pub data: Vec<Vec<u8>>,
    pub path: Option<PathBuf>,
}

impl Image {
    pub fn new(width: u32, height: u32, mip_levels: u32, data: Vec<Vec<u8>>, path: Option<PathBuf>) -> Self {
        return Self {
            data: data,
            path: path,
            width: width,
            height: height,
            mip_levels: mip_levels,
        };
    }

    pub fn from_raw(data: Vec<u8>, mip_levels: u32) -> Self {
        log::debug!("Loading image");
        let mut image = image::load_from_memory(&data).unwrap_or_else(|_| {
            log::warn!("Unknown image format, trying TGA");
            image::load_from_memory_with_format(&data, image::ImageFormat::Tga).expect("Unknown image format")
        });

        let width = image.width();
        let height = image.height();

        let effective_mip_levels = if Self::has_enough_detail(&image) {
            mip_levels
        } else {
            log::warn!("Texture lacks sufficient detail, skipping mipmap generation");
            1
        };

        let mut levels = Vec::new();
        log::debug!("Generating mipmaps");
        // FIXME: sometimes data.len() != width * height * 4
        for level in 0..effective_mip_levels {
            image = image.resize(
                width >> level,
                height >> level,
                image::imageops::FilterType::Triangle,
            );
            levels.push(image.to_rgba8().to_vec());
            log::debug!("Resizing: {:?}", ((width >> level), (height >> level)));
        }

        return Self::new(width, height, effective_mip_levels, levels, None);
    }

    fn has_enough_detail(image: &image::DynamicImage) -> bool {
        const STD_DEV_THRESHOLD: f64 = 6.0;

        let rgba = image.to_rgba8();
        let (w, h) = (rgba.width() as u64, rgba.height() as u64);
        let total_pixels = w * h;
        if total_pixels == 0 {
            return false;
        }

        let mut count: u64 = 0;
        let mut sum: f64 = 0.0;
        let mut sum_sq: f64 = 0.0;
        let step = ((total_pixels / 4096).max(1)) as usize;

        for (i, pixel) in rgba.pixels().enumerate() {
            if i % step != 0 {
                continue;
            }
            let [r, g, b, _a] = pixel.0;
            let luminance = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
            sum += luminance;
            sum_sq += luminance * luminance;
            count += 1;
        }

        if count == 0 {
            return false;
        }

        let mean = sum / count as f64;
        let variance = (sum_sq / count as f64 - mean * mean).max(0.0);
        let std_dev = variance.sqrt();

        std_dev >= STD_DEV_THRESHOLD
    }

    pub fn from_path(path: &PathBuf, mip_levels: u32) -> Self {
        let data = std::fs::read(path).expect("Failed to read image file");
        let mut image = Self::from_raw(data, mip_levels);
        image.path = Some(path.into());

        return image;
    }

    pub fn empty() -> Self {
        Self {
            width: 1,
            height: 1,
            path: None,
            mip_levels: 1,
            data: vec![vec![0, 0, 0, 0]],
        }
    }
}
