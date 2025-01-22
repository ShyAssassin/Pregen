use std::env;
use std::path::Path;
use serde_json::Value;
use std::process::Command;

const DXC_GITHUB: &str = "microsoft/DirectXShaderCompiler";

fn main() {
    // cfg! does not work in build scripts
    let target = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    if target != "wasm" {
        let out = env::var("OUT_DIR").expect("Out dir not found");

        let out_dir = Path::new(&out);
        // NOTE: if the way build dirs are structured changes, this breaks
        let crate_dir = out_dir.parent().expect("Failed to get crate dir");
        let build_dir = crate_dir.parent().expect("Failed to get build dir");
        let profile_dir = build_dir.parent().expect("Failed to get profile dir");
        let target_dir = profile_dir.parent().expect("Failed to get targets dir");

        let app_bin_dir = profile_dir.join("bin");
        let dxc_source = build_dir.join("gfx-dxc");
        let dxc_archive = target_dir.join("dxc.zip");

        if !dxc_archive.exists() {
            println!("DXC not found, downloading to {:?}", dxc_archive);
            let output = std::process::Command::new("curl")
                .arg("-s").arg(format!("https://api.github.com/repos/{}/releases/latest", DXC_GITHUB))
                .arg("-H").arg("User-Agent: Mozilla/5.0 Firefox/133.0")
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
        }

        if !dxc_source.exists() {
            println!("Extracting DXC to {:?}", dxc_source);
            std::fs::create_dir_all(&dxc_source).expect("Failed to create dxc dir");
            #[cfg(target_os = "windows")]
            let output = std::process::Command::new("tar")
                .arg("-xf").arg(dxc_archive)
                .arg("-C").arg(&dxc_source)
            .output().expect("Failed to execute tar");
            #[cfg(not(target_os = "windows"))]
            let output = std::process::Command::new("unzip")
                .arg("-q").arg(dxc_archive)
                .arg("-d").arg(&dxc_source)
            .output().expect("Failed to execute unzip");

            if !output.status.success() {
                panic!("Failed to extract dxc.zip: {}", String::from_utf8_lossy(&output.stderr));
            }
        }

        if !app_bin_dir.join("dxcompiler.dll").exists() || !app_bin_dir.join("dxil.dll").exists() {
            println!("Copying DXC DLLs to {:?}", app_bin_dir);
            std::fs::create_dir_all(&app_bin_dir).expect("Failed to create app bin dir");
            std::fs::copy(dxc_source.join("bin/x64/dxil.dll"), app_bin_dir.join("dxil.dll")).expect("Failed to copy dxil.dll");
            std::fs::copy(dxc_source.join("bin/x64/dxcompiler.dll"), app_bin_dir.join("dxcompiler.dll")).expect("Failed to copy dxcompiler.dll");
        }
    }
}
