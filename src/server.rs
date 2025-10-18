const MAX_CONNECTIONS = 10;
const METHOD_LENGTH = 8;
const URL_LENGTH = 128;
const LOCAL_HOST = "127.0.0.1";

struct http_request {
    method: String,
    url: String,
}

impl http_request {
    pub fn new(method: String, url: String) -> Self {
        http_request {
            method,
            url,
        }
    }
    
}
