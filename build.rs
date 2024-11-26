use std::env;
use std::path::Path;
use serde_json::Value;
use std::process::Command;

const DXC_GITHUB: &str = "microsoft/DirectXShaderCompiler";

fn main() {
    let target = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("Manifest dir not found");
    let profile = env::var("PROFILE").expect("Profile not found");
    let out = env::var("OUT_DIR").expect("Out dir not found");

    let build_dir = Path::new(&out);
    let target_dir = Path::new(&target);
    let _manifest_dir = Path::new(&manifest);
    let profile_dir = target_dir.join(profile);

    let dxc_source = build_dir.join("dxc");
    let app_bin_dir = profile_dir.join("bin");
    let dxc_archive = target_dir.join("dxc.zip");

    if !dxc_archive.exists() {
        println!("DXC not found, downloading...");
        let api_url = format!("https://api.github.com/repos/{}/releases/latest", DXC_GITHUB);
        let output = Command::new("curl")
            .arg("-s").arg(&api_url)
            .arg("-H").arg("User-Agent: rust-build-script")
            .output().expect("Failed to execute curl command");
        if !output.status.success() {
            panic!("Failed to get latest release: {}", String::from_utf8_lossy(&output.stderr));
        }
        let release: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse JSON");
        for asset in release["assets"].as_array().expect("Assets is not an array") {
            let name = asset["name"].as_str().expect("Name is not a string");
            let download_url = asset["browser_download_url"].as_str().expect("Download URL is not a string");
            if name.starts_with("dxc_") && name.ends_with(".zip") && !name.contains("clang") {
                println!("Downloading {} to {}", name, dxc_archive.display());
                let output = Command::new("curl")
                    .arg("-L").arg(download_url)
                    .arg("-o").arg(dxc_archive.to_str().unwrap())
                    .output().expect("Failed to execute curl command");
                if !output.status.success() {
                    panic!("Failed to download file: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        println!("DXC downloaded successfully");
    }
    if !dxc_source.exists() {
        std::fs::create_dir(&dxc_source).expect("Failed to create dxc dir");
        #[cfg(target_os = "windows")]
        let output = Command::new("tar")
            .arg("-xf").arg(dxc_archive)
            .arg("-C").arg(&dxc_source)
            .output().expect("Failed to execute tar command");
        #[cfg(not(target_os = "windows"))]
        let output = Command::new("unzip")
            .arg("-q").arg(dxc_archive)
            .arg("-d").arg(&dxc_source)
            .output().expect("Failed to execute unzip command");

        if !output.status.success() {
            panic!("Failed to extract dxc.zip: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    if !app_bin_dir.join("dxcompiler.dll").exists() || !app_bin_dir.join("dxil.dll").exists() {
        std::fs::create_dir(&app_bin_dir).expect("Failed to create bin dir");
        std::fs::copy(dxc_source.join("bin/x64/dxil.dll"), app_bin_dir.join("dxil.dll")).expect("Failed to copy dxil.dll");
        std::fs::copy(dxc_source.join("bin/x64/dxcompiler.dll"), app_bin_dir.join("dxcompiler.dll")).expect("Failed to copy dxcompiler.dll");
    }
}
