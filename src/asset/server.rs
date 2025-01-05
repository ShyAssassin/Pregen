use super::Image;
use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;

pub struct AssetServer {
    pub images: HashMap<PathBuf, Arc<Image>>,
}

impl AssetServer {
    pub fn new() -> Self {
        todo!()
    }

    pub fn load_image(&mut self, path: impl Into<PathBuf>) -> Arc<Image> {
        let path = path.into();
        return self.images.get(&path).cloned().unwrap_or_else(|| {
            let image = Arc::new(Image::from_path(&path, 5));
            self.images.insert(path, image.clone());
            return image;
        });
    }
}
