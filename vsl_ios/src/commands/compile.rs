use clap::{ArgMatches, Error, ErrorKind};
use std::process::{Command, Output};

use artifacts::{Artifact};

pub fn cli(matches: &ArgMatches) {
    let target_sys = matches.value_of("SYSTEM")
                            .map(|string: &str| string.to_owned())
                            .expect("No system received");

    let target_platform = matches.value_of("PLATFORM")
                                 .map(|string: &str| string.to_owned())
                                 .expect("No platform received");

    let platform_version = matches
        .value_of("platform-version")
        .map_or_else(|| {
            // Use xcrun helper command to help use extract the latest SDK version if it's not
            // provided
            Command::new("xcrun")
                    .args(&["--sdk", target_platform.as_str(), "--show-sdk-version"])
                    .output()
                    .ok()
                    // Not using this until stable in rust
                    // .filter(|output: &Output| output.status.success())
                    .and_then(|output: Output| if output.status.success() { Some(output) } else { None })
                    .and_then(|output: Output| String::from_utf8(output.stdout).ok())
                    // We'll need to remove the trailing newline from println()
                    .and_then(|string: String| Some(string.trim().to_string()))
                    .expect("Could not determine platform version, please explicitly specify.")
        }, |value: &str| value.to_string());

    let artifact = Artifact::new(target_sys, target_platform, platform_version, "x86_64".to_string());

    info!("Using artifact {}", artifact.get_name());
    info!("Using triple {}", artifact.get_triple());

    info!("Path is {}", artifact.get_path().display());
    if !matches.is_present("force-compile") && artifact.get_path().as_path().exists() {
        error!("Already compiled. Use `-f` to force re-compile");
        ::std::process::exit(1)
    }

    artifact.get_path_for_temp("value")
}
