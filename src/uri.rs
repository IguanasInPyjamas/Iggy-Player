
pub fn setup_pipeline(path:&std::string::String) -> gstreamer::Element {
    //let mut filepath :std::string::String = "".to_string();
    //For use in unit tests.
    //let path = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_cropped_multilingual.webm";
    //Relatively naive, needs to be future proofed
    let mut filepath = std::string::String::from("");
    if path.contains("https") || path.contains("http"){

        filepath = path.replace(" ","%20").to_string();
    } else {
        filepath = get_uri_from_path(&path)
    };
    let pipe = format!("playbin uri={} name=play video-sink='autovideosink' audio-sink='autoaudiosink'", filepath);
    let pipeline = gstreamer::parse_launch(&pipe).unwrap();
    return pipeline;
}

pub fn get_uri_from_path(filepath: &str) -> String {
    let path = std::path::Path::new(filepath);
    let uri = url::Url::from_file_path(path).unwrap();
    println!("{}",uri.as_str().to_string());
    return uri.as_str().to_string();


}
