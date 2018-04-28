#[macro_use] extern crate log;
extern crate pretty_env_logger;

extern crate clap;
extern crate colored;

mod commands;
mod artifacts;

use clap::{App, Arg, AppSettings, SubCommand};
use colored::*;

fn main() {
    pretty_env_logger::init();

    let matches = App::new("vsl-ios")
                          .about("manages VSL iOS files")
                          .setting(AppSettings::SubcommandRequiredElseHelp)
                          .subcommand(SubCommand::with_name("create")
                                      .about("Creates a VSL Obj-C linkage template"))
                          .subcommand(SubCommand::with_name("compile")
                                      .arg(Arg::with_name("SYSTEM")
                                           .help("The system, e.g. `ios` or `macosx`")
                                           .required(true)
                                           .index(1))
                                      .arg(Arg::with_name("PLATFORM")
                                           .help("The target platform, e.g. `iphonesimulator`, `iphoneos`, or `darwin`")
                                           .required(true)
                                           .index(2))
                                      .arg(Arg::with_name("platform-version")
                                           .short("p")
                                           .long("platform-version")
                                           .help("The platform version of darwin, iOS, etc.")
                                           .takes_value(true))
                                      .arg(Arg::with_name("arch")
                                           .short("a")
                                           .long("arch")
                                           .help("The *target* architecture, e.g. x86_64, armv7")
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
        _ => {}
    }
}
