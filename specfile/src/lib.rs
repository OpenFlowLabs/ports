mod errors;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate error_chain;

use pest::Parser;
use errors::*;

#[derive(Parser)]
#[grammar = "specfile.pest"]
struct SpecFileParser;

#[derive(Default, Debug)]
pub struct SpecFile {
    name: String,
    version: String,
    release: String,
    summary: String,
    license: String,
    description: String,
    prep_script: String,
    build_script: String,
    install_script: String,
    files: Vec<String>,
    changelog: String,
}

pub fn parse(file_contents: String) -> Result<SpecFile> {
    let pairs = SpecFileParser::parse(Rule::file, &file_contents)?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair can be converted to an iterator of the tokens which make it up:
        match pair.as_rule() {
            Rule::variable => {
                let variable_parts: Vec<&str> =
                    pair.clone().into_inner().map(|p| p.as_str()).collect();
                println!("{}: {}", variable_parts[0], variable_parts[1]);
            }
            Rule::section => {
                for section_rule in pair.clone().into_inner() {
                    match section_rule.as_rule() {
                        Rule::description_section => {
                            println!("Description:");
                            for description_rule in section_rule.clone().into_inner() {
                                match description_rule.as_rule() {
                                    Rule::section_line => print!("{}", description_rule.as_str()),
                                    _ => println!(
                                        "Unknown description: {:?}",
                                        description_rule.as_rule()
                                    ),
                                }
                            }
                        }
                        Rule::prep_section => {
                            println!("Prep:");
                            for prep_rule in section_rule.clone().into_inner() {
                                match prep_rule.as_rule() {
                                    Rule::section_line => print!("{}", prep_rule.as_str()),
                                    _ => println!(
                                        "Unknown description: {:?}",
                                        prep_rule.as_rule()
                                    ),
                                }
                            }
                        }
                        Rule::build_section => {
                            println!("Build:");
                            for build_rule in section_rule.clone().into_inner() {
                                match build_rule.as_rule() {
                                    Rule::section_line => print!("{}", build_rule.as_str()),
                                    _ => println!(
                                        "Unknown description: {:?}",
                                        build_rule.as_rule()
                                    ),
                                }
                            }
                        }
                        Rule::install_section => {
                            println!("Install:");
                            for install_rule in section_rule.clone().into_inner() {
                                match install_rule.as_rule() {
                                    Rule::section_line => print!("{}", install_rule.as_str()),
                                    _ => println!(
                                        "Unknown description: {:?}",
                                        install_rule.as_rule()
                                    ),
                                }
                            }
                        }
                        Rule::files_section => {
                            println!("Files:");
                            for files_rule in section_rule.clone().into_inner() {
                                match files_rule.as_rule() {
                                    Rule::section_line => print!("{}", files_rule.as_str()),
                                    _ => println!(
                                        "Unknown description: {:?}",
                                        files_rule.as_rule()
                                    ),
                                }
                            }
                        }
                        Rule::changelog_section => {
                            println!("Changelog:");
                            for changelog_rule in section_rule.clone().into_inner() {
                                match changelog_rule.as_rule() {
                                    Rule::section_line => print!("{}", changelog_rule.as_str()),
                                    _ => println!(
                                        "Unknown description: {:?}",
                                        changelog_rule.as_rule()
                                    ),
                                }
                            }
                        }
                        _ => print!("Rule:    {:?}", section_rule.as_rule()),
                    }
                }
            }
            Rule::EOI => {}
            _ => print!("Rule:    {:?}", pair.as_rule()),
        }
        println!();
    }

    Ok(SpecFile::default())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::parse;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_parse() {
        let contents = fs::read_to_string("./test_data/simple.spec");
        match contents {
            Ok(file) => {
                let spec = parse(file);
                assert!(spec.is_err(), "parsing error {:?}", spec)
            },
            Err(e) => panic!("error: {:}", e)
        }
    }
}
