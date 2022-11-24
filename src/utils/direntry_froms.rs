//! DirEntry Utilities

use crate::prelude::*;

use std::fs::DirEntry;


impl TryFrom<Wrapper<&DirEntry>> for String {

    type Error = Error;

    fn try_from(value: Wrapper<&DirEntry>) -> Result<String> {
        value.0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| {
                Error::Generic(f!("Invalid path: {:?}", value.0))
            })
    }
}
