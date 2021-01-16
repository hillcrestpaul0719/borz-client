use url::{Url, ParseError};

pub struct APIFetcher {
    path: Url,
    name: String,
}

impl APIFetcher {
    pub fn new(path: Url, name: String) -> APIFetcher {
        APIFetcher {
            path,
            name,
        }
    } 
}
