use std::path;
use url::Url;

pub fn get_uri_from_path(filepath: &str) -> String {
    let path = std::path::Path::new(filepath);
    let uri = url::Url::from_file_path(path).unwrap();
    println!("{}",uri.as_str().to_string());
    return uri.as_str().to_string();


}