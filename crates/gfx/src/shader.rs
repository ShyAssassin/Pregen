use crate::Device;
use std::sync::Mutex;
use std::path::PathBuf;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum ShaderStage {
    Vertex,
    Compute,
    Fragment,
}

pub struct Shader {
    pub name: Option<String>,
    pub shader: Option<String>,
    pub sources: Option<Vec<PathBuf>>,
    pub module: Mutex<wgpu::ShaderModule>,
    pub stages: Vec<(ShaderStage, String)>,
}

impl Shader {
    pub fn new(device: &Device, name: Option<String>, path: PathBuf) -> Self {
        let name = name.unwrap_or("Unnamed Shader".into());

        if path.extension().unwrap() == "wgsl" {
            let (shader, sources) = Self::parse_shader(path);
            let stages = Self::discover_entries(shader.clone());
            let module = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&name),
                source: wgpu::ShaderSource::Wgsl(shader.as_str().into()),
            });

            return Self {
                stages: stages,
                name: Some(name),
                shader: Some(shader),
                sources: Some(sources),
                module: Mutex::new(module),
            }
        } else if path.extension().unwrap() == "spv" {
            log::info!("Loading spirv shader: {:?}", &path);
            let module = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&name),
                source: wgpu::util::make_spirv(&std::fs::read(&path).unwrap()),
            });

            return Self {
                shader: None,
                name: Some(name),
                stages: Vec::new(),
                sources: Some(vec![path]),
                module: Mutex::new(module),
            }
        }

        panic!("Unsupported shader format");
    }

    pub fn reload(&self) {}

    pub fn get_entry(&self, stage: ShaderStage) -> Option<&str> {
        for entry in &self.stages {
            if entry.0 == stage {
                return Some(&entry.1);
            }
        }
        return None;
    }

    fn discover_entries(source: String) -> Vec<(ShaderStage, String)> {
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

    fn parse_shader(source: PathBuf) -> (String, Vec<PathBuf>) {
        log::info!("Parsing shader {:?}", source);
        let shader = std::fs::read_to_string(&source).unwrap();
        let mut imports: Vec<PathBuf> = shader.lines()
            .filter(|line| line.starts_with("#import"))
            .map(|line| {
                let path = line.split_whitespace().last().expect("Expected a path after #import");
                let path = path.trim_matches('"');
                source.parent().and_then(|p| Some(p.join(path))).unwrap_or_else(|| {
                    log::error!("Failed to get parent directory for {:?}", source);
                    PathBuf::from(path)
                })
            })
        .collect();

        let mut shader = shader.lines()
            .filter(|line| !line.starts_with("#import"))
            .collect::<Vec<_>>()
        .join("\n");

        let sources = imports.iter()
            .map(|path| Self::parse_shader(path.to_path_buf()))
        .collect::<Vec<_>>();
        imports.insert(0, source.clone());
        imports.extend(sources.iter().flat_map(|(_, sources)| sources.clone()));

        shader = sources.iter()
            .map(|(shader, _)| shader.as_str())
            .collect::<Vec<_>>()
        .join("\n") + "\n" + &shader;

        imports.sort();
        imports.dedup();
        return (shader, imports);
    }
}

impl std::fmt::Debug for Shader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shader")
            .field("name", &self.name)
            .field("stages", &self.stages)
            .field("sources", &self.sources)
            .field("module", &(&self.module as *const _))
            .field("shader", &self.shader)
        .finish()
    }
}
