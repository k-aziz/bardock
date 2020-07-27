// use serde_derive::Deserialize;

// // Referenced from cargo 1.46.0-nightly (c26576f9a 2020-06-23)
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "kebab-case")]
// pub struct TomlManifest {
//     cargo_features: Option<Vec<String>>,
//     package: Option<Box<TomlProject>>,
//     project: Option<Box<TomlProject>>,
//     profile: Option<TomlProfiles>,
//     lib: Option<TomlLibTarget>,
//     bin: Option<Vec<TomlBinTarget>>,
//     example: Option<Vec<TomlExampleTarget>>,
//     test: Option<Vec<TomlTestTarget>>,
//     bench: Option<Vec<TomlTestTarget>>,
//     dependencies: Option<BTreeMap<String, TomlDependency>>,
//     dev_dependencies: Option<BTreeMap<String, TomlDependency>>,
//     #[serde(rename = "dev_dependencies")]
//     dev_dependencies2: Option<BTreeMap<String, TomlDependency>>,
//     build_dependencies: Option<BTreeMap<String, TomlDependency>>,
//     #[serde(rename = "build_dependencies")]
//     build_dependencies2: Option<BTreeMap<String, TomlDependency>>,
//     features: Option<BTreeMap<String, Vec<String>>>,
//     target: Option<BTreeMap<String, TomlPlatform>>,
//     replace: Option<BTreeMap<String, TomlDependency>>,
//     patch: Option<BTreeMap<String, BTreeMap<String, TomlDependency>>>,
//     workspace: Option<TomlWorkspace>,
//     badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
// }


use serde::{Serialize, Deserialize};
use std::{path::PathBuf, fs::File, io::{Read, BufReader}};
use super::errors::BardockResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
    pub cargo_features: Option<toml::Value>,
    pub package: Option<toml::Value>,
    pub project: Option<toml::Value>,
    pub profile: Option<toml::Value>,
    pub lib: Option<toml::Value>,
    pub bin: Option<toml::Value>,
    pub example: Option<toml::Value>,
    pub test: Option<toml::Value>,
    pub bench: Option<toml::Value>,
    pub dependencies: Option<toml::Value>,
    pub dev_dependencies: Option<toml::Value>,
    #[serde(rename = "dev_dependencies")]
    pub dev_dependencies2: Option<toml::Value>,
    pub build_dependencies: Option<toml::Value>,
    #[serde(rename = "build_dependencies")]
    pub build_dependencies2: Option<toml::Value>,
    pub features: Option<toml::Value>,
    pub target: Option<toml::Value>,
    pub replace: Option<toml::Value>,
    pub patch: Option<toml::Value>,
    pub workspace: Option<toml::Value>,
    pub badges: Option<toml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
}

pub fn read_manifest(manifest_path: &PathBuf) -> BardockResult<CargoToml> {

    let manifest = File::open(manifest_path)?;
    let mut buf_reader = BufReader::new(manifest);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // let toml_content = r#"
    //       [package]
    //       name = "your_package"
    //       version = "0.1.0"
    //       authors = ["You! <you@example.org>"]

    //       [dependencies]
    //       serde = "1.0"
    //       "#;

    let package_info: CargoToml = toml::from_str(&contents)?;

    Ok(package_info)
}


#[cfg(test)]
mod tests {
    use super::read_manifest;
    use std::{env, path::Path, fs::File};
    use anyhow::Context;
    use crate::util::errors::BardockResult;
    use std::io::Write;

    #[test]
    fn test_read_manifest() -> BardockResult<()> {
        let cwd = env::current_dir().with_context(|| "couldn't get the current directory of the process")?;
        let src_path = &Path::new(&cwd).join("src/bardock/util");

        let manifest = read_manifest(src_path)?;

        println!("\nMANIFEST: {:?}", manifest);

        let new_man = toml::to_string(&manifest)?;
        
        let mut new_outfile = File::create(src_path.join("outfile.toml"))?;
        new_outfile.write_all(
            new_man.as_bytes()
        )?;
        println!("\nNEW MANIFEST:\n{}", new_man);


        Ok(())
    }
}