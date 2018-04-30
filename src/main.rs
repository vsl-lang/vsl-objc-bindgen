#[macro_use] extern crate log;
extern crate pretty_env_logger;

extern crate clap;
extern crate clang;

#[macro_use] mod xcrun;
#[macro_use] mod macros;
mod commands;
mod artifacts;
// mod vsl_ast;

use clap::{App, Arg, AppSettings, SubCommand};

fn main() {
    pretty_env_logger::init();

    let matches = App::new("vsl-ios")
                          .about("manages VSL iOS files")
                          .setting(AppSettings::SubcommandRequiredElseHelp)
                          .subcommand(SubCommand::with_name("create")
                                      .about("Creates a VSL iOS/macOS .app template"))
                          .subcommand(SubCommand::with_name("gen")
                                      .about("Generates a (Objective)-C(++) and VSL binding files")
                                      .arg(Arg::with_name("SYSTEM")
                                               .help("The system, e.g. `ios` or `macosx`")
                                               .required(true)
                                               .index(1))
                                      .arg(Arg::with_name("PLATFORM")
                                               .help("The target platform, e.g. `iphonesimulator`, `iphoneos`, or `darwin`")
                                               .required(true)
                                               .index(2))
                                      .arg(Arg::with_name("ENTRY_HEADER")
                                               .help("A header references all symbols to compile")
                                               .required(true)
                                               .index(3))
                                      .arg(Arg::with_name("C_OUTPUT_FILE")
                                               .help("The output (Objective)-C(++) file. Run with `compile` to generate application code.")
                                               .required(true)
                                               .index(4))
                                      .arg(Arg::with_name("VSL_OUTPUT_FILE")
                                               .help("The output/binding VSL file")
                                               .required(true)
                                               .index(5))
                                      .arg(Arg::with_name("platform-version")
                                               .short("p")
                                               .long("platform-version")
                                               .help("The platform version of darwin, iOS, etc.")
                                               .takes_value(true))
                                      .arg(Arg::with_name("SYMBOLS")
                                               .help("A list of all symbols to compile.")
                                               .short("s")
                                               .long("symbol")
                                               .takes_value(true)
                                               .multiple(true))
                                      .arg(Arg::with_name("frameworks")
                                               .help("A list of all Objective-C frameworks to use.")
                                               .short("F")
                                               .long("framework")
                                               .takes_value(true)
                                               .multiple(true))
                                      .arg(Arg::with_name("includes")
                                               .help("Includes needed to run.")
                                               .short("i")
                                               .long("include")
                                               .takes_value(true)
                                               .multiple(true)))
                          .subcommand(SubCommand::with_name("compile")
                                      .arg(Arg::with_name("SYSTEM")
                                               .help("The system, e.g. `ios` or `macosx`")
                                               .required(true)
                                               .index(1))
                                      .arg(Arg::with_name("PLATFORM")
                                               .help("The target platform, e.g. `iphonesimulator`, `iphoneos`, or `darwin`")
                                               .required(true)
                                               .index(2))
                                      .arg(Arg::with_name("FILES")
                                               .help("Objective-C `.m` files to compile")
                                               .multiple(true)
                                               .required(true)
                                               .index(3))
                                      .arg(Arg::with_name("platform-version")
                                               .short("p")
                                               .long("platform-version")
                                               .help("The platform version of darwin, iOS, etc.")
                                               .takes_value(true))
                                      .arg(Arg::with_name("force-compile")
                                               .short("f")
                                               .long("force")
                                               .help("Force or always rebuild"))
                                      .about("Recompiles VSL <-> Obj-C linkage bridges"))
                          .get_matches();

    match matches.subcommand() {
        ("compile", Some(matches)) => commands::compile::cli(matches),
        ("create", Some(matches)) => commands::create::cli(matches),
        ("gen", Some(matches)) => commands::gen::cli(matches),
        _ => {}
    }
}
