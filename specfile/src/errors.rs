use error_chain::error_chain;
use crate::Rule;

error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Pest(pest::error::Error<Rule>);
    }
}