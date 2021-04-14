mod workspace;
mod sources;

#[macro_use]
extern crate failure_derive;

use clap::app_from_crate;
use clap::{Arg, App};
use std::fs;
use specfile::parse;
use specfile::macros;
use std::collections::HashMap;
use crate::workspace::Workspace;

mod errors {
    use failure::Error;
    use std::result::Result as StdResult;

    pub type Result<T> = StdResult<T, Error>;
}

use errors::Result;

pub enum Verbose{
    Off,
    Some,
    On,
    Debug
}

fn main() {
    let opts = app_from_crate!().arg(Arg::new("config")
        .short('c')
        .long("config")
        .value_name("CONFIG_FILE")
        .about("Sets a custom config file")
        .takes_value(true).default_value("/etc/ports.yaml")
    ).arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .takes_value(false)
            .about("Sets the level of verbosity")
    ).subcommand(App::new("package")
            .about("makes a package out of the spec file")
            .arg(Arg::new("target")
                .short('t')
                .long("target")
                .value_name("REPOSITORY")
                .about("set the target repository to output the package to")
                .default_value("./repository")
            ).arg(Arg::new("SPECFILE")
                .about("Sets the spec file to use")
                .required(true)
                .index(1)
            )
    ).get_matches();

    if let Some(c) = opts.value_of("CONFIG_FILE") {
        println!("Value for config: {}", c);
    }

    let verbose = match opts.occurrences_of("v") {
        0 => Verbose::Off,
        1 => Verbose::Some,
        2 => Verbose::On,
        3 | _ => Verbose::Debug,
    };

    if let Some(ref package_opts) = opts.subcommand_matches("package") {
        let target = package_opts.value_of("target").expect("Target must always be a variable was the default_value removed from code?");
        let spec_file_name = package_opts.value_of("SPECFILE").expect("Specfile is required, do not remove required from that arg");
        match run_package_command(spec_file_name, target) {
            Err(e) => {
                println!("error: {}", e);

                std::process::exit(1);
            },
            _ => (),
        }
    }

    match run_package_command("../ports.spec", "./repository") {
        Err(e) => {
            panic!("error: {}", e);
        },
        _ => (),
    }

}

fn run_package_command(spec_file: &str, _target: &str) -> Result<()> {
    let content_string = fs::read_to_string(spec_file)?;
    let spec = parse(content_string)?;
    let mut ws = Workspace::new("")?;
    let downloaded = ws.get_sources(spec.sources)?;
    ws.unpack_all_sources(downloaded)?;

    let mut macro_map= HashMap::<String, String>::new();
    for ws_macro in ws.get_macros() {
        macro_map.insert(ws_macro.0, ws_macro.1);
    }

    let mp = macros::MacroParser {
        macros: macro_map
    };

    let build_script = mp.parse(spec.build_script)?;
    ws.build(build_script)?;
    ws.package(spec.files)?;

    Ok(())
}
