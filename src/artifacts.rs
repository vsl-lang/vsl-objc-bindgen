use std::path::{PathBuf, Path};
use std::process::Command;
use std::fs;

pub struct Artifact {
    sys: String,
    platform: String,
    version: String,
    arch: String
}

impl Artifact {
    pub fn new(sys: String, platform: String, version: String, arch: String) -> Artifact {
        Artifact {
            sys: sys,
            platform: platform,
            version: version,
            arch: arch
        }
    }

    pub fn get_platform(&self) -> &String {
        &self.platform
    }

    pub fn get_sys(&self) -> &String {
        &self.sys
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_triple(&self) -> String {
        format!("{}-apple-{}{}", self.arch, self.sys, self.version)
    }

    pub fn get_name(&self) -> String {
        format!("vsl-{}", self.get_triple())
    }

    pub fn get_sdk_name(&self) -> String {
        format!("{}{}", self.platform, self.version)
    }

    pub fn get_sdk_path(&self) -> String {
        let sdk_name = self.get_sdk_name();

        xcrun!(
            &["--sdk", sdk_name.as_str(), "--show-sdk-path"],
            "Could not find SDK path"
        )
    }

    pub fn get_platform_path(&self) -> String {
        let sdk_name = self.get_sdk_name();

        xcrun!(
            &["--sdk", sdk_name.as_str(), "--show-sdk-path"],
            "Could not find provided platform path"
        )
    }

    pub fn get_path(&self) -> PathBuf {
        let mut artifact_path = Artifact::get_artifact_root();
        artifact_path.push(format!("{}.bc", &self.get_name()));
        artifact_path
    }

    pub fn get_path_for_temp(&self, temp: &str) -> PathBuf {
        let mut artifact_path = Artifact::get_temporary_root();
        artifact_path.push(format!("{}-{}.bc", &self.get_name(), temp));
        artifact_path
    }

    pub fn get_temporary_root() -> PathBuf {
        let mut artifact_path = Artifact::get_artifact_root();
        artifact_path.push(".tmp");
        if let Result::Err(err) = fs::create_dir_all(artifact_path.as_path()) {
            error!("could not create temporary directory: {}", err);
            ::std::process::exit(1);
        }
        artifact_path
    }

    pub fn get_artifact_root() -> PathBuf {
        let artifact_path = PathBuf::from("artifacts");
        if let Result::Err(err) = fs::create_dir_all(artifact_path.as_path()) {
            error!("could not create artifact directory: {}", err);
            ::std::process::exit(1);
        }
        artifact_path
    }
}
