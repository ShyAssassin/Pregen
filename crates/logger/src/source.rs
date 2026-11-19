#[derive(Debug, Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct LogSource {
    pub toplevel: String,
    pub submodules: Vec<String>,
}

impl LogSource {
    pub fn new(toplevel: &str, submodules: Vec<&str>) -> Self {
        return Self {
            toplevel: toplevel.to_string(),
            submodules: submodules.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn from_target(target: &str) -> Self {
        let mut parts = target.split("::").into_iter();
        let toplevel = parts.next().unwrap_or("").to_string();
        let submodules = parts.map(|sm| String::from(sm)).filter(|sm| {
            // The last submodule specified is treated as a wildcard
            // Also nuke relative module path specifiers, stinky!
            !["*", ":", ".", "/", "\\"].contains(&sm.as_str())
        }).collect();

        return Self {
            toplevel: toplevel,
            submodules: submodules,
        }
    }

    pub fn matches(&self, target: &str) -> bool {
        let source = Self::from_target(target);

        if source.submodules.len() < self.submodules.len() {
            return false;
        } else if self.toplevel != source.toplevel  {
            return false;
        }

        for (i, submodule) in self.submodules.iter().enumerate() {
            if submodule != &source.submodules[i] {
                return false;
            }
        }

        return true;
    }
}
