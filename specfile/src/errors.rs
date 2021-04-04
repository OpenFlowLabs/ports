use error_chain::error_chain;
use crate::Rule;
use crate::macros;

error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Pest(pest::error::Error<Rule>);
        PestMacro(pest::error::Error<macros::Rule>);
    }
}