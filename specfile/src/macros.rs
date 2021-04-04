use pest::Parser;
use crate::errors::*;

#[derive(Parser)]
#[grammar = "macro.pest"]
struct InternalMacroParser;

#[derive(Default, Debug)]
pub struct MacroParser {
    pub proto_dir: String,
}

impl MacroParser {
    pub fn parse(&self ,raw_string: String) -> Result<String> {
        let mut return_string = String::new();

        for (i, line) in raw_string.lines().enumerate() {
            let mut replaced_line = line.clone().to_string();
            let pairs = InternalMacroParser::parse(Rule::text_with_macros, &line)?;

            for pair in pairs {
                match pair.as_rule() {
                    Rule::text_with_macros => {
                        for inner in pair.into_inner() {
                            match inner.as_rule() {
                                Rule::spec_macro => {
                                    for macro_pair in inner.clone().into_inner() {
                                        match macro_pair.as_rule() {
                                            Rule::macro_name => {
                                                replaced_line = line.replacen(inner.as_str(), self.get_variable(macro_pair.as_str()), 1)
                                            },
                                            Rule::macro_parameter => println!("macro parameter: {}", macro_pair.as_str()),
                                            _ => panic!(
                                                "Unexpected macro match: {:?}",
                                                macro_pair.as_rule()
                                            )
                                        }
                                    }
                                }
                                Rule::text => (),
                                _ => panic!(
                                    "Unexpected inner match: {:?}",
                                    inner.as_rule()
                                )
                            }
                        }
                    },
                    _ => panic!(
                        "Unexpected match: {:?}",
                        pair.as_rule()
                    )
                }
            }
            if i == 0 {
                return_string += &replaced_line;
            } else {
                return_string += "\n";
                return_string += &replaced_line;
            }
        }

        Ok(return_string)
    }

    fn get_variable(&self, macro_name: &str) -> &str {
        if macro_name == "proto_dir" {
            return self.proto_dir.as_str();
        }
        ""
    }
}

