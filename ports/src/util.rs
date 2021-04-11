use crate::errors::*;

pub fn convert_to_str(os_str: Option<&str>) -> Result<String> {
    match os_str {
        Some(s) => Ok(s.into()),
        None => Err(ErrorKind::Msg(format!("Can not convert {:?} to string", os_str)).into())
    }
}