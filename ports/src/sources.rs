use crate::errors::*;
use url::Url;
use std::path::Path;
use crate::util::convert_to_str;

pub struct Source {
    pub url: Url,
    pub local_name: String
}

impl Source {
    pub fn new(url_string: &str, local_base: &str) -> Result<Source> {
        let url = Url::parse(url_string)?;
        let str = url.path().to_owned();
        let path = Path::new(&str);
        match path.file_name() {
            Some(fname) => {
                let s = convert_to_str(fname.to_str())?;
                return Ok(Source{url, local_name: local_base.to_string() + "/" + &s})
            }
            None => Err(ErrorKind::CantCreateSource(url_string.into()).into()),
        }
    }
}