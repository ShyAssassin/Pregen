use std::fs;
use crate::Device;
use std::path::PathBuf;
use std::fmt::Formatter;

#[derive(Eq, PartialEq)]
#[derive(Debug, Copy, Clone)]
pub enum ShaderStage {
    Vertex,
    Compute,
    Fragment,
}

#[derive(Eq, PartialEq)]
#[derive(Debug, Copy, Clone)]
pub enum ShaderSource {
    Wgsl, Glsl,
    Slang, Spirv,
}

// TODO: should this have internal mutability?
pub struct Shader<'a> {
    pub name: String,
    pub device: &'a Device,
    pub path: Option<PathBuf>,
    pub source: Option<String>,
    pub source_type: ShaderSource,
    pub module: wgpu::ShaderModule,
    pub imports: Option<Vec<PathBuf>>,
    pub stages: Vec<(ShaderStage, String)>,
}

impl<'a> Shader<'a> {
    pub fn new(device: &'a Device, name: String, path: PathBuf) -> Self {
        log::info!("Loading shader: {:?}", &path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("wgsl") => {
                let (shader, imports) = Self::parse_shader(&path);
                let stages = Self::discover_entries(&shader);
                let module = device.wgpu_create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(&name),
                    source: wgpu::ShaderSource::Wgsl(shader.as_str().into()),
                });

                return Self {
                    name: name,
                    module: module,
                    stages: stages,
                    device: &device,
                    path: Some(path),
                    source: Some(shader),
                    imports: Some(imports),
                    source_type: ShaderSource::Wgsl,
                };
            }
            Some("spv") => {
                let module = device.wgpu_create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(&name),
                    source: wgpu::util::make_spirv(&std::fs::read(&path).unwrap()),
                });

                return Self {
                    name: name,
                    source: None,
                    imports: None,
                    module: module,
                    device: &device,
                    path: Some(path),
                    stages: Vec::new(),
                    source_type: ShaderSource::Spirv,
                };
            }
            _ => panic!("Unsupported shader format"),
        };
    }

    pub fn reload(&mut self) {
        // TODO: probably not this, this is cursed
        *self = Shader::new(self.device, self.name.clone(), self.path.clone().unwrap());
    }

    pub fn get_entry(&self, stage: ShaderStage) -> Option<&str> {
        for entry in &self.stages {
            if entry.0 == stage {
                return Some(&entry.1);
            }
        }
        return None;
    }

    fn discover_entries(source: &String) -> Vec<(ShaderStage, String)> {
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

    fn parse_shader(source: &PathBuf) -> (String, Vec<PathBuf>) {
        let shader = match fs::read_to_string(&source) {
            Ok(content) => content,
            Err(err) => {
                // TODO: indicate whether the error is from an import or the main shader
                log::error!("Failed to read shader file {:?}: {}", source, err);
                // TODO: fallback to error.wgsl as a default shader?
                panic!("Failed to read shader file {:?}: {}", source, err);
            }
        };
        // Collect all import paths from the shader source
        let mut imports: Vec<PathBuf> = shader.lines()
            .filter(|line| line.starts_with("#import"))
            .map(|line| {
                // Extract the path from the import line
                let path = line.split_whitespace().last().expect("Expected a path after #import");
                let path = path.trim_matches('"'); // Remove surrounding quotes from the path
                // Resolve the path relative to the shader source's parent directory
                source.parent().and_then(|p| Some(p.join(path))).unwrap_or_else(|| {
                    log::error!("Failed to get parent directory for {:?}", source);
                    log::error!("Falling back to using the path as-is");
                    // We are probably fucked but go on with it anyways
                    PathBuf::from(path)
                })
            })
        .collect();

        // Remove import keywords
        let mut shader = shader.lines()
            .filter(|line| !line.starts_with("#import"))
            .collect::<Vec<_>>()
        .join("\n");

        // Parse imported shaders
        // NOTE: will stack overflow on circular imports
        let sources = imports.iter()
            .map(|path| Self::parse_shader(&path))
        .collect::<Vec<_>>();
        // Flatten the sources into a single string
        imports.insert(0, source.clone());
        imports.extend(sources.iter().flat_map(|(_, sources)| sources.clone()));

        // Prepend the imported shaders to the main shader
        // FIXME: does not account for duplicate functions
        shader = sources.iter()
            .map(|(shader, _)| shader.as_str())
            .collect::<Vec<_>>()
        .join("\n") + "\n" + &shader;

        imports.sort();
        imports.dedup();
        return (shader, imports);
    }
}

impl<'a> std::fmt::Debug for Shader<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shader")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("stages", &self.stages)
            .field("imports", &self.imports)
            .field("source type", &self.source_type)
            .field("device", &(&self.device as *const _))
            .field("module", &(&self.module as *const _))
            .field("source", &(&self.source as *const _))
        .finish()
    }
}
