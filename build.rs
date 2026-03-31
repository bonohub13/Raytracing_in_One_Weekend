use std::{fs, path::PathBuf, process::Command};

const COMPILER: &str = "slangc";
const SHADERS_DIR: &str = "shaders";
const SHADERS_OUT_SUBDIR: &str = "spv";
const SHADERS_SRC_SUFFIX: &str = "slang";
const SHADERS_DST_SUFFIX: &str = "spv";

fn main() {
    let shader_paths = query_shader_path();
    let shader_dir = PathBuf::from(SHADERS_DIR);
    let out_dir = shader_dir.join(SHADERS_OUT_SUBDIR);

    shader_paths.iter().for_each(|shader_path| {
        println!("cargo:rerun-if-changed={}", shader_path.to_string_lossy());
        let dst_path = out_dir.join(
            shader_path
                .to_string_lossy()
                .split("/")
                .last()
                .expect("Failed to get file name: {shader_path:?}")
                .strip_suffix(SHADERS_SRC_SUFFIX)
                .map(|prefix| format!("{prefix}{SHADERS_DST_SUFFIX}"))
                .unwrap_or_else(|| shader_path.to_string_lossy().to_string()),
        );
        let status = Command::new(COMPILER)
            .arg(shader_path)
            .arg("-I")
            .arg("shaders")
            .arg("-target")
            .arg("spirv")
            .arg("-profile")
            .arg("sm_6_0")
            .arg("-entry")
            .arg("main")
            .arg("-o")
            .arg(&dst_path)
            .arg("-O3")
            .status()
            .expect("Failed to execute slangc.");

        if !status.success() {
            panic!("Shader compilation failed! ({shader_path:?})")
        }
    });
}

fn query_shader_path() -> Vec<PathBuf> {
    fs::read_dir(SHADERS_DIR)
        .expect("Failed to query shader files: ({SHADER_DIR})")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();

            path.is_file() && path.to_string_lossy().ends_with(SHADERS_SRC_SUFFIX)
        })
        .map(|entry| entry.path())
        .collect()
}
