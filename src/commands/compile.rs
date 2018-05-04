use clap::{ArgMatches};
use std::process::Command;

use artifacts::{Artifact};
use std::path::{Path, PathBuf};
use std::ffi::{OsStr};

pub fn cli(matches: &ArgMatches) {
    let target_sys = matches.value_of("SYSTEM")
                            .map(|string: &str| string.to_owned())
                            .expect("No system received");

    let target_platform = matches.value_of("PLATFORM")
                                 .map(|string: &str| string.to_owned())
                                 .expect("No platform received");

    let platform_version = matches
        .value_of("platform-version")
        .map_or_else(|| Artifact::infer_version(&target_platform), |value: &str| value.to_string());

    let frameworks: Vec<String> = matches.values_of_lossy("frameworks").unwrap_or(vec![])
        .into_iter()
        .flat_map(|framework| vec!["-framework".to_string(), framework])
        .collect();

    let artifact = Artifact::new(target_sys, target_platform, platform_version, "x86_64".to_string());

    info!("Using artifact {}", artifact.get_name());
    info!("Using triple {}", artifact.get_triple());

    let out_file = artifact.get_path();
    let out_file = out_file.as_path();
    info!("Path is {}", out_file.display());

    // Check if path is already compiled
    if !matches.is_present("force-compile") && out_file.exists() {
        error_exit!("Already compiled. Use `-f` to force re-compile");
    }

    // Convert output file to path
    let out_file = match out_file.to_str() {
        Some(value) => value,
        None => {
            error_exit!("Could not generate output file name.")
        }
    };

    // Compile all files
    let files = match matches.values_of_lossy("FILES") {
        Some(value) => value,
        None => {
            error_exit!("No values provided")
        }
    };

    let sdk_path = artifact.get_sdk_path();
    let platform_path = artifact.get_platform_path();

    let compiled_files: Vec<PathBuf> = files.iter().filter_map(|source_path: &String| -> Option<PathBuf> {
        let source_file = source_path.clone();
        let source_file = Path::new(&source_file);

        let source_name = source_file.file_stem()
            .and_then(|name: &OsStr| name.to_str())
            .expect(&format!("Could not parse name for file {}", source_file.display()));

        let temp_file_buf = artifact.get_path_for_temp(source_name);
        let temp_file = temp_file_buf.as_path();

        let temp_path = temp_file.to_str()
            .expect(&format!("Failed to obtain path of {}", temp_file.display()));

        info!("Processing file {} ({}) to {}", source_path, source_name, temp_file.display());

        let mut clang_args = artifact.get_comp_args();
        clang_args.push(source_path.to_string());
        clang_args.extend_from_slice(&["-emit-llvm".to_string(), "-S".to_string(), "-o".to_string(), temp_path.to_string()]);
        clang_args.extend_from_slice(&frameworks);

        info!("Compiling with {:?}", clang_args);

        match Command::new("clang")
                .args(clang_args)
                .output() {
            Ok(output) => {
                if !output.status.success() {
                    warn!("Failed to compile {}: {}", source_file.display(), String::from_utf8_lossy(&output.stderr));
                    None
                } else {
                    info!("Succesfully compiled {}", source_file.display());
                    Some(temp_file_buf.to_owned())
                }
            }
            Err(err) => {
                warn!("Failed to compile {}: {}", source_file.display(), err);
                None
            }
        }
    }).collect();

    info!("Succesfully compiled {} file(s).", compiled_files.len());

    // Archive these files
    match Command::new("llvm-link")
            .args(compiled_files)
            .arg(format!("-o={}", out_file))
            .output() {
        Ok(output) => {
            if !output.status.success() {
                error_exit!("Failed to link: {}", String::from_utf8_lossy(&output.stderr));
            } else {
                info!("Successfully linked");
            }
        }
        Err(_err) => {
            error_exit!("Failed to run link step.");
        }
    };
}
