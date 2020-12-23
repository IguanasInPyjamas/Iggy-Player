use gstreamer;
use gstreamer::prelude::*;
use device_query::Keycode;
use std::io;
use std::io::Write;
use glib;
use glib::prelude::*;


use crate::stream_info;

pub fn volume_down(stream_info: &mut stream_info::_StreamInfo, prev_position: gstreamer::ClockTime){
    if stream_info.volume >= 0.05{
        let new_volume = stream_info.volume - 0.05;
        stream_info.playbin.set_property("volume", &new_volume.to_value()).expect("Volume could not be set");
        stream_info.volume = new_volume;
        std::thread::sleep(std::time::Duration::from_millis(50))
    } else{
        println!("Volume too low, can't go lower");
    }
}

pub fn volume_up(stream_info: &mut stream_info::_StreamInfo, prev_position: gstreamer::ClockTime){
    if stream_info.volume <= 0.95{
        let new_volume = stream_info.volume + 0.05;
        stream_info.playbin.set_property("volume",&new_volume.to_value()).expect("Volume could not be set");
        stream_info.volume = new_volume;
        std::thread::sleep(std::time::Duration::from_millis(50))
    } else{
        println!("Volume too high, can't go higher");
    }
}

//pub fn stream_speed_up(stream_info: &mut stream_info::_StreamInfo, prev_position: gstreamer::ClockTime){

//}

//pub fn stream_speed_down(stream_info: &mut stream_info::_StreamInfo, prev_position: gstreamer::ClockTime){

//}

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
        stream_info.playbin.set_property("mute", &false).expect("Property could not be set");
        std::thread::sleep(std::time::Duration::from_millis(200));
        stream_info.mute = false;
    } else
    {
        stream_info.playbin.set_property("mute", &true).expect("Property could not be set");
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

pub fn check_keypress(keys: &Vec<device_query::Keycode>,position:gstreamer::ClockTime,mut stream_info: &mut stream_info::_StreamInfo) -> gstreamer::ClockTime {
    let mut prev_position = position;
    if keys.contains(&Keycode::Space) {
        pause(&stream_info);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if keys.contains(&Keycode::A) &&
        stream_info.seek_enabled &&
        !stream_info.playing &&
        prev_position > 5 * gstreamer::SECOND {
        prev_position = seek_left(&mut stream_info, prev_position);

    }

    if keys.contains(&Keycode::D) &&
        stream_info.seek_enabled && !stream_info.playing {
        prev_position = seek_right(&mut stream_info, prev_position);

    }

    if stream_info.playing {
        let position = stream_info
            .playbin
            .query_position::<gstreamer::ClockTime>().expect("Couldn't query");


        if stream_info.duration == gstreamer::CLOCK_TIME_NONE {
            stream_info.duration = stream_info.playbin.query_duration().expect("Couldn't query");
        }
        println!("Position {} / {}", position, stream_info.duration);
        io::stdout().flush().unwrap();


        if keys.contains(&Keycode::D) && stream_info.seek_enabled {
            stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not set to paused");
            prev_position = seek_right(&mut stream_info, prev_position);
            stream_info.playbin.set_state(gstreamer::State::Playing).expect("Could not set to playing");
            std::thread::sleep(std::time::Duration::from_millis(250))
        }

        if keys.contains(&Keycode::A) && stream_info.seek_enabled && prev_position > 5 * gstreamer::SECOND {
            stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not set to paused");
            prev_position = seek_left(&mut stream_info, prev_position);
            stream_info.playbin.set_state(gstreamer::State::Playing).expect("Could not set to playing");
            std::thread::sleep(std::time::Duration::from_millis(250))
        }



        if keys.contains(&Keycode::V) && stream_info.n_audio_streams > 0{
            switch_audio(&mut stream_info);
            prev_position = position;
        }
    }
    if keys.contains(&Keycode::M) {
        mute(&mut stream_info);
        prev_position = position;
    }

    if keys.contains(&Keycode::S) && stream_info.n_subtitles > 0{
        switch_subs(&mut stream_info);
        prev_position = position;
    }

    if keys.contains(&Keycode::U) &&
        stream_info.playing{
        volume_down(&mut stream_info, prev_position)
    }

    if keys.contains(&Keycode::I) &&
        stream_info.playing{
        volume_up(&mut stream_info, prev_position)
    }

    return prev_position
}
