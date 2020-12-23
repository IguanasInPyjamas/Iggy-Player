use gstreamer;
use gstreamer::prelude::*;
use glib;
use glib::prelude::*;

pub struct _StreamInfo {
    pub playbin: gstreamer::Element, //Stream.
    pub playing: bool, //Is it playing or paused
    pub seek_enabled: bool, //Can stream seek?
    pub terminate: bool, //End?
    pub duration: gstreamer::ClockTime,
    pub n_video_streams: i32, //Number of video streams
    pub n_audio_streams: i32, //Number of audio streams
    pub n_subtitles: i32, //Number of subtitle files

    pub current_video_stream: i32,
    pub current_audio_stream: i32,
    pub current_subtitle: i32,

    pub mute: bool,
    pub volume: f64,
    pub speed: f64,
}

pub fn populate_from_playbin(stream_info: &mut _StreamInfo){
    stream_info.n_audio_streams = stream_info.playbin.get_property("n-audio").unwrap().get_some::<i32>().unwrap();
    stream_info.n_video_streams = stream_info.playbin.get_property("n-video").unwrap().get_some::<i32>().unwrap();
    stream_info.n_subtitles = stream_info.playbin.get_property("n-text").unwrap().get_some::<i32>().unwrap();

    stream_info.current_audio_stream = stream_info.playbin.get_property("current-audio").unwrap().get_some::<i32>().unwrap();
    stream_info.current_video_stream = stream_info.playbin.get_property("current-video").unwrap().get_some::<i32>().unwrap();
    stream_info.current_subtitle = stream_info.playbin.get_property("current-text").unwrap().get_some::<i32>().unwrap();
    println!("{}, {}, {}", stream_info.current_audio_stream, stream_info.current_video_stream, stream_info.current_subtitle);
    println!("{}, {}, {}", stream_info.n_audio_streams, stream_info.n_video_streams, stream_info.n_subtitles);

}

pub fn initialise_stream(pipeline:gstreamer::Element) -> _StreamInfo {
    let stream_info = _StreamInfo {
        playbin: pipeline,
        playing: false,
        terminate: false,
        seek_enabled: false,
        duration: gstreamer::CLOCK_TIME_NONE,
        n_video_streams: 0,
        n_audio_streams: 0,
        n_subtitles: 0,
        current_audio_stream: 0,
        current_video_stream: 0,
        current_subtitle: 0,
        mute: false,
        volume: 1.0,
        speed: 1.0,

    };
    return stream_info
}
