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
            width: width,
            height: height,
            mip_levels: mip_levels,
            path: path.map(|p| p.into()),
        };
    }

    pub fn from_raw(data: Vec<u8>, mip_levels: u32) -> Self {
        println!("Loading image");
        let image = image::load_from_memory(&data).unwrap_or_else(|_|  {
            println!("Unknown image format, trying TGA");
            image::load_from_memory_with_format(&data, image::ImageFormat::Tga).expect("Unknown image format")
        });

        println!("Generating mipmaps");
        let mut levels = Vec::new();
        // FIXME: sometimes data.len() != width * height * 4
        for level in 0..mip_levels {
            let data = image.clone().resize(
                image.width() >> level,
                image.height() >> level,
                image::imageops::FilterType::Triangle,
            ).to_rgba8();
            levels.push(data.to_vec());
        };
        dbg!("Image loaded");
        return Self::new(image.width(), image.height(), mip_levels, levels, None);
    }

    pub fn from_path(path: &PathBuf, mip_levels: u32) -> Self {
        let data = std::fs::read(path).expect("Failed to read image file");
        let mut image = Self::from_raw(data, mip_levels);
        image.path = Some(path.into());

        return image;
    }
}
