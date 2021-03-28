extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "specfile.pest"]
struct SpecFile;

fn main() {
    let specfile = r###"Name:       hello-world
Version:    1
Release:    1
Summary:    Most simple RPM package
License:    FIXME

%description
This is my first RPM package, which does nothing.
This is a second description line.

%prep
# we have no source, so nothing here

%build
cat > hello-world.sh <<EOF
#!/usr/bin/bash
echo Hello world
EOF

%install
mkdir -p %{buildroot}/usr/bin/
install -m 755 hello-world.sh %{buildroot}/usr/bin/hello-world.sh

%files
/usr/bin/hello-world.sh

%changelog
# let's skip this for now

"###;

    let pairs = SpecFile::parse(Rule::file, specfile).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair can be converted to an iterator of the tokens which make it up:
        match pair.as_rule() {
            Rule::variable => {
                let variable_parts: Vec<&str> = pair.clone().into_inner().map(|p| p.as_str()).collect();
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
                                    _ => println!("Unknown description: {:?}", description_rule.as_rule())
                                }
                            }
                        }
                        Rule::prep_section => {
                            println!("Prep:");
                            for description_rule in section_rule.clone().into_inner() {
                                match description_rule.as_rule() {
                                    Rule::section_line => print!("{}", description_rule.as_str()),
                                    _ => println!("Unknown description: {:?}", description_rule.as_rule())
                                }
                            }
                        }
                        Rule::build_section => {
                            println!("Build:");
                            for description_rule in section_rule.clone().into_inner() {
                                match description_rule.as_rule() {
                                    Rule::section_line => print!("{}", description_rule.as_str()),
                                    _ => println!("Unknown description: {:?}", description_rule.as_rule())
                                }
                            }
                        }
                        Rule::install_section => {
                            println!("Install:");
                            for description_rule in section_rule.clone().into_inner() {
                                match description_rule.as_rule() {
                                    Rule::section_line => print!("{}", description_rule.as_str()),
                                    _ => println!("Unknown description: {:?}", description_rule.as_rule())
                                }
                            }
                        }
                        Rule::files_section => {
                            println!("Files:");
                            for description_rule in section_rule.clone().into_inner() {
                                match description_rule.as_rule() {
                                    Rule::section_line => print!("{}", description_rule.as_str()),
                                    _ => println!("Unknown description: {:?}", description_rule.as_rule())
                                }
                            }
                        }
                        Rule::changelog_section => {
                            println!("Changelog:");
                            for description_rule in section_rule.clone().into_inner() {
                                match description_rule.as_rule() {
                                    Rule::section_line => print!("{}", description_rule.as_str()),
                                    _ => println!("Unknown description: {:?}", description_rule.as_rule())
                                }
                            }
                        }
                        _ =>  print!("Rule:    {:?}", section_rule.as_rule()),
                    }
                }
            }
            Rule::EOI => {}
            _ =>  print!("Rule:    {:?}", pair.as_rule()),
        }
        println!("");
    }
}