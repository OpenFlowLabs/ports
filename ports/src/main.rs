mod errors;

use errors::*;
use clap::app_from_crate;
use clap::{Arg, App};
use std::fs;
use specfile::parse;

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

    match opts.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

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

}

fn run_package_command(spec_file: &str, _target: &str) -> Result<()> {
    let content_string = fs::read_to_string(spec_file)?;
    let _spec = parse(content_string)?;
    Ok(())
}
