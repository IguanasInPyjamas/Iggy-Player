
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
}