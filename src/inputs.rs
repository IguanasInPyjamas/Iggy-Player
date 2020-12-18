use gstreamer;
use gstreamer::prelude::*;
use glib;

use crate::stream_info;

pub fn seek_right(stream_info: &mut stream_info::_StreamInfo, prev_position: gstreamer::ClockTime) -> gstreamer::ClockTime {
    let mut previous = prev_position;
    stream_info.playbin.seek_simple(gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT, previous + 5 * gstreamer::SECOND).expect("Failed to seek");
    previous += 5 * gstreamer::SECOND;
    return previous;
}

pub fn seek_left(stream_info: &mut stream_info::_StreamInfo, prev_position: gstreamer::ClockTime) -> gstreamer::ClockTime {
    let mut previous = prev_position;
    stream_info.playbin.seek_simple(gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT, prev_position - 5 * gstreamer::SECOND).expect("Failed to seek");
    previous -= 5 * gstreamer::SECOND;
    return previous

}

pub fn mute(stream_info: &mut stream_info::_StreamInfo){
    if stream_info.mute == true {
        stream_info.playbin.set_property("mute", &false);
        std::thread::sleep(std::time::Duration::from_millis(200));
        stream_info.mute = false;
    } else
    {
        stream_info.playbin.set_property("mute", &true);
        std::thread::sleep(std::time::Duration::from_millis(200));
        stream_info.mute = true;
    }
}

pub fn switch_audio(stream_info: &mut stream_info::_StreamInfo){
    let new_stream = (stream_info.current_audio_stream + 1) % stream_info.n_audio_streams;
    stream_info.playbin.set_property("current-audio", &new_stream).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    stream_info.current_audio_stream = new_stream;
}

pub fn switch_subs(stream_info: &mut stream_info::_StreamInfo){
    //BUG: When the subs go they don't come back
    let new_stream = (stream_info.current_subtitle + 1) % stream_info.n_subtitles;
    stream_info.playbin.set_property("current-text", &new_stream).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    stream_info.current_audio_stream = new_stream;
}

//fn toggle_subs(stream_info){
//    0;

//}

pub fn pause(stream_info: &stream_info::_StreamInfo){
    if stream_info.playing == true {
        println! {"Pausing"}
        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not pause");
    } else {
        stream_info.playbin.set_state(gstreamer::State::Playing)
            .expect("Could not resume");
    }

}
