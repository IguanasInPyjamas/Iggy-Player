use std::io;
use std::io::Write;
use gstreamer;
use device_query::{DeviceQuery, DeviceState, Keycode};
use gstreamer::prelude::*;
use glib;

use crate::stream_info;
use crate::inputs;
use crate::events;
use crate::uri;

pub fn player_loop(args:Vec<String>) {
    let args = args;
    //args 0 has the reference to binaries, the first filepath will then be in point 1
    //TODO: Encorporate more use of args to play multiple files, preferably after a --playlist tag

    let path = &args[1];
    gstreamer::init().unwrap();
    let device_state = DeviceState::new();
    //TODO Implement a filepath to URI function and then this doesn't have to be hardcoded, this is just a test case.
    //TODO Use the GNU online videos for unit testing purposes.
    // TODO Implement unit tests.
    let mut filepath :std::string::String = "".to_string();
    //For use in unit tests.
    //let path = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_cropped_multilingual.webm";

    //Relatively naive, needs to be future proofed
    if path.contains("https") || path.contains("http"){
        path.replace(" ","%20");
        filepath = path.to_string();
    } else {
        filepath = uri::get_uri_from_path(&path)
    };

    let pipe = format!("playbin uri={} name=play video-sink='autovideosink' audio-sink='autoaudiosink'", filepath);



    let mut pipeline = gstreamer::parse_launch(&pipe).unwrap();

    pipeline.set_state(gstreamer::State::Playing).expect("Unable to set to playing");



    let bus = pipeline.get_bus().unwrap();

    let mut prev_position = 0 * gstreamer::SECOND;
    let mut stream_info = stream_info::_StreamInfo {
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

    };

    let (flags, flags_class) = events::setup_flags_class(&stream_info);
    //These can then be set and unset using this value.


    let flags = flags_class.builder_with_value(flags).unwrap()
        .set_by_nick("audio")
        .build()
        .unwrap();
    stream_info.playbin.set_property("flags", &flags).unwrap();

    events::setup_connects(&stream_info);

    //TODO: Sleep is here to ensure that the video is buffered before the streams are queried, we can probably get this from a signal return instead of time.
    std::thread::sleep(std::time::Duration::from_millis(4000));
    stream_info::populate_from_playbin(&mut stream_info);

    while !stream_info.terminate {
        let msg = bus.timed_pop(25 * gstreamer::MSECOND);
        let keys: Vec<Keycode> = device_state.get_keys();

        match msg {
            Some(msg) => {
                events::handle_message(&mut stream_info, &msg);
            }
            None => {
                if keys.contains(&Keycode::Space) {
                    inputs::pause(&stream_info);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }

                if keys.contains(&Keycode::A) &&
                    stream_info.seek_enabled && !stream_info.playing {
                    prev_position = inputs::seek_left(&mut stream_info, prev_position);

                }

                if keys.contains(&Keycode::D) &&
                    stream_info.seek_enabled && !stream_info.playing {
                    prev_position = inputs::seek_right(&mut stream_info, prev_position);

                }
                if stream_info.playing {
                    let mut position = stream_info
                        .playbin
                        .query_position::<gstreamer::ClockTime>().expect("Couldn't query");


                    if stream_info.duration == gstreamer::CLOCK_TIME_NONE {
                        stream_info.duration = stream_info.playbin.query_duration().expect("Couldn't query");
                    }
                    println!("Position {} / {}", position, stream_info.duration);
                    io::stdout().flush().unwrap();


                    if keys.contains(&Keycode::D) && stream_info.seek_enabled {
                        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not set to paused");
                        prev_position = inputs::seek_right(&mut stream_info, prev_position);
                        stream_info.playbin.set_state(gstreamer::State::Playing).expect("Could not set to playing");
                    }

                    if keys.contains(&Keycode::A) && stream_info.seek_enabled {
                        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not set to paused");
                        prev_position = inputs::seek_left(&mut stream_info, prev_position);
                        stream_info.playbin.set_state(gstreamer::State::Playing).expect("Could not set to playing");
                    }

                    prev_position = position;

                    if keys.contains(&Keycode::V) {
                        inputs::switch_audio(&mut stream_info);
                    }
                }
                if keys.contains(&Keycode::M) {
                    inputs::mute(&mut stream_info);
                }

                if keys.contains(&Keycode::S){
                    inputs::switch_subs(&mut stream_info);
                }
            }
        }
    }
    stream_info.playbin.set_state(gstreamer::State::Null).expect("Unable to set state");



}
