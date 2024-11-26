use std::fs;
use std::fmt::Debug;
use std::sync::LazyLock;
use std::path::{Path, PathBuf};
use std::sync::{RwLock, RwLockReadGuard};

static SHADER_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/");
    if !path.join("shaders/").exists() {
        let exe_path = std::env::current_exe().unwrap();
        path = exe_path.parent().unwrap().to_path_buf();
    }
    return path;
});

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderStage {
    Vertex,
    Compute,
    Fragment
}

pub struct Shader {
    pub name: String,
    pub src_path: PathBuf,
    pub module: RwLock<wgpu::ShaderModule>,
    pub stages: Vec<(ShaderStage, String)>,
}

impl Shader {
    pub fn new(device: &wgpu::Device, name: Option<&str>, source_path: impl Into<PathBuf>) -> Self {
        let path = Path::new(&*SHADER_PATH).join(source_path.into());
        let source = fs::read_to_string(&path).unwrap();
        let stages = Self::parse_wgsl(&source);

        let name = name.unwrap_or(path.file_stem().unwrap().to_str().unwrap());
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        return Self {
            name: name.to_string(),
            module: RwLock::new(module),
            stages: stages,
            src_path: path,
        };
    }

    pub fn get_entry(&self, stage: ShaderStage) -> Option<&str> {
        for (s, entry) in &self.stages {
            if s == &stage {
                return Some(entry.as_str());
            }
        }
        return None;
    }

    pub fn as_raw(&self) -> RwLockReadGuard<wgpu::ShaderModule> {
        return self.module.read().unwrap();
    }

    pub fn reload(&self, device: &wgpu::Device) {
        let source = fs::read_to_string(&self.src_path).unwrap();
        let mut module = self.module.write().unwrap();
        *module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&self.name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    }

    fn parse_wgsl(source: &str) -> Vec<(ShaderStage, String)> {
        let mut stages = Vec::new();
        for (i, line) in source.lines().enumerate() {
            if line.starts_with("@") {
                let stage = match &line.to_lowercase()[..] {
                    "@vertex" => ShaderStage::Vertex,
                    "@compute" => ShaderStage::Compute,
                    "@fragment" => ShaderStage::Fragment,
                    _ => continue,
                };
                let entry_point = source.lines().nth(i + 1).unwrap().trim();
                let fn_index = entry_point.find("fn").unwrap() + 3;
                let open_paren_index = entry_point.find("(").unwrap();
                let name = entry_point[fn_index..open_paren_index].trim();
                stages.push((stage, name.to_string()));
            }
        }
        return stages;
    }
}

impl Debug for Shader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shader")
            .field("name", &self.name)
            .field("stages", &self.stages)
            .field("src_path", &self.src_path)
            .field("module", &(&self.module as *const _))
        .finish()
    }
}
