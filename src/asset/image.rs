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
        let mut image = image::load_from_memory(&data).unwrap_or_else(|_|  {
            log::warn!("Unknown image format, trying TGA");
            image::load_from_memory_with_format(&data, image::ImageFormat::Tga).expect("Unknown image format")
        });

        let mut levels = Vec::new();
        log::debug!("Generating mipmaps");
        // FIXME: sometimes data.len() != width * height * 4
        let width = image.width();
        let height = image.height();
        for level in 0..mip_levels {
            image = image.resize(
                width >> level,
                height >> level,
                image::imageops::FilterType::Triangle,
            );
            levels.push(image.to_rgba8().to_vec());
            log::debug!("Resizing: {:?}", ((width >> level),(height >> level)));
        };
        return Self::new(width, height, mip_levels, levels, None);
    }

    pub fn from_path(path: &PathBuf, mip_levels: u32) -> Self {
        let data = std::fs::read(path).expect("Failed to read image file");
        let mut image = Self::from_raw(data, mip_levels);
        image.path = Some(path.into());

        return image;
    }
}
