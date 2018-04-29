macro_rules! xcrun {
    ($args:expr, $msg:expr) => {
        {
            use std::process::{Command, Output};
            match Command::new("xcrun")
                    .args($args)
                    .output()
                    .ok()
                    // Not using this until stable in rust
                    // .filter(|output: &Output| output.status.success())
                    .and_then(|output: Output| if output.status.success() { Some(output) } else { None })
                    .and_then(|output: Output| String::from_utf8(output.stdout).ok())
                    // We'll need to remove the trailing newline from println()
                    .and_then(|string: String| Some(string.trim().to_string())) {
                Some(value) => value,
                None => {
                    error!($msg);
                    ::std::process::exit(1)
                }
            }
        }
    };
}
