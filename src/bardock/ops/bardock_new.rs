use crate::Config;
use crate::util::errors::BardockResult;
use crate::util::toml::{CargoToml, read_manifest};
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;
use std::io::Write;
use toml::{Value, value::Map};


// struct Options<'a> {
//     path: &'a Path,
//     name: &'a str,
//     authors: Option<Vec<&'a str>>
// }


pub fn new(path: &str, config: &Config) -> BardockResult<Option<std::process::Output>> {
    log::info!("Initialising new cargo project");

    // let mk_opts = 

    let result = match Command::new("cargo").arg("new").arg(path).output() {
        Err(e) => {
            anyhow::bail!("failed to call cargo: {}", e);
        }
        Ok(output) => {
            if !output.status.success() {
                anyhow::bail!("cargo failed: {:?}", from_utf8(&output.stderr));
            } else {
                output
            }
        }
    };

    make(config, path)?;

    Ok(Some(result))
}

fn make(config: &Config, path: &str) -> BardockResult<()> {
    let src_path = &config.cwd().join(&path);
    // let package_name = src_path.file_name();

    let lib_path = Path::new(&src_path).join("src/lib.rs");
    let manifest_path = Path::new(&src_path).join("Cargo.toml");

    mklib(&lib_path)?;

    let mut manifest = read_manifest(&manifest_path)
        .with_context(|| "Unable to read manifest")?;
    let updated_manifest = match get_updated_manifest(&mut manifest) {
        Ok(manifest) => {
            toml::to_string(manifest)?
        }
        Err(e) => { anyhow::bail!("Failed to generate updated manifest: {}", e) }
    };
    
    let mut file = File::create(&manifest_path)?;
    file.write_all(updated_manifest.as_bytes())
        .with_context(|| "Unable to write manifest data to file")?;

    if cfg!(target_os = "macos") {
        macos_conf(src_path)
            .with_context(|| "Unable to create .cargo/config file for MacOS")?;
    }

    cleanup_files(src_path)?;

    log::info!("New cargo project generated at {:?}", src_path.display());
    Ok(())
}

/// Remove main.rs created by `cargo new`
fn cleanup_files(src_path: &PathBuf) -> BardockResult<()> {
    let path = Path::new(&src_path).join("src/main.rs");
    fs::remove_file(&path).with_context(|| format!("main.rs not found at path {:?}", path.display()))
}

/// Adds .cargo/config file for additional linker arguments required for MacOS
fn macos_conf(src_path: &PathBuf) -> BardockResult<()> {
    fs::create_dir(Path::new(src_path).join(".cargo/"))?;

    let mut file = File::create(Path::new(src_path).join(".cargo/config"))?;

    file.write_all(
        r#"[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]"#.as_bytes()
    )?;
    Ok(())
}

/// Create new lib.rs with pyo3 template code
fn mklib(lib_path: &PathBuf) -> BardockResult<()> {
    let mut file = File::create(&lib_path)?;

    file.write_all(
        "\
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;

    Ok(())
}".as_bytes())?;

    Ok(())
}

fn get_updated_manifest(manifest: &mut CargoToml) -> BardockResult<&mut CargoToml> {
    let mut pyo3_dependency_map = Map::new();
    pyo3_dependency_map.insert(
        String::from("features"), 
        Value::Array(vec![Value::String(String::from("extension-module"))])
    );
    pyo3_dependency_map.insert(
        String::from("version"), 
        Value::String(String::from("0.11.1"))
    );

    if let Some( Value::Table(ref mut libs) ) = manifest.lib {
        update_manifest_lib(libs)?;
    } else {
        let mut libs = Map::new();
        update_manifest_lib(&mut libs)?;
        manifest.lib = Some(Value::from(libs));
    }

    if let Some( Value::Table(ref mut dependencies) ) = manifest.dependencies {
        update_manifest_dependencies(dependencies, pyo3_dependency_map)?;
    } else {
        let mut dependencies = Map::new();
        update_manifest_dependencies(&mut dependencies, pyo3_dependency_map)?;
        manifest.dependencies = Some(Value::from(dependencies));
    }

    Ok(manifest)
}

fn update_manifest_lib(lib: &mut Map<String, Value>) -> BardockResult<&mut Map<String, Value>> {
    lib.insert(
        String::from("name"),
        Value::String(String::from("some package name"))
    );
    lib.insert(
        String::from("path"),
        Value::String(String::from("path to lib.rs"))
    );
    lib.insert(
        String::from("crate-type"),
        Value::Array(vec![Value::String(String::from("cdylib"))])
    );

    Ok(lib)
}

fn update_manifest_dependencies(
    dependencies_map: &mut Map<String, Value>,
    pyo3_map: Map<String, Value>
) -> BardockResult<&mut Map<String, Value>> {
    dependencies_map.insert(
        String::from("pyo3"), 
        Value::from(pyo3_map)
    );
    // Need to add a normal dependency or the formatting fucks up when serialized for some reason
    dependencies_map.insert(
        String::from("log"),
        Value::from("0.4.8")
    );

    Ok(dependencies_map)
}


#[cfg(test)]
mod tests {

    use super::get_updated_manifest;
    use crate::util::{errors::BardockResult, read_manifest, cwd};
    use std::path::Path;

    #[test]
    fn test_update_manifest() -> BardockResult<()> {
        let cwd = &cwd()?;
        let manifest_path = Path::new(cwd).join("src/bardock/ops/Cargo.toml");
        println!("{:?}", manifest_path);

        let mut manifest = read_manifest(&manifest_path)?;
        let new_man = get_updated_manifest(&mut manifest)?;

        println!("\nMANIFEST: \n{}", toml::to_string(new_man)?);

        Ok(())
    }
}