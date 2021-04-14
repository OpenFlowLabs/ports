use crate::errors::Result;
use url::Url;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug, Fail)]
pub enum SourceError {
    #[fail(display = "can't create source from url: {}", url)]
    CantCreateSource {
        url: String,
    }
}

pub struct Source {
    pub url: Url,
    pub local_name: String
}

impl Source {
    pub fn new(url_string: &str, local_base: &str) -> Result<Source> {
        let url = Url::parse(url_string)?;
        let path = url.path().to_owned();
        let path_vec: Vec<_> = path.split("/").collect();
        match path_vec.last() {
            Some(str) => {
                let local_name = str.clone();
                Ok(Source {
                    url,
                    local_name: local_base.clone().to_owned() + "/" + local_name,
                })
            }
            None => Err(SourceError::CantCreateSource {url: url.into_string()})?
        }
    }
}