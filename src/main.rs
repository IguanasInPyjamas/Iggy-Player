use std::io;
use std::io::Write;
use gstreamer;
use device_query::{DeviceQuery, DeviceState, Keycode};
use gstreamer::prelude::*;
use glib;

struct _StreamInfo {
    playbin: gstreamer::Element, //Stream.
    playing: bool, //Is it playing or paused
    seek_enabled: bool, //Can stream seek?
    terminate: bool, //End?
    duration: gstreamer::ClockTime,
    n_video_streams: i32, //Number of video streams
    n_audio_streams: i32, //Number of audio streams
    n_subtitles: i32, //Number of subtitle files

    current_video_stream: i32,
    current_audio_stream: i32,
    current_subtitle: i32
}

struct stream_switcher{
    stream: i32,
}

fn main() {
    gstreamer::init().unwrap();
    let device_state = DeviceState::new();
    //TODO Implement a filepath to URI function and then this doesn't have to be hardcoded, this is just a test case.
    //TODO Use the GNU online videos for unit testing purposes.
    // TODO Implement unit tests.
    //let filepath = "file:///media/mep19mj/Anime%201/TV%20Shows/Psych/Psych.Season.5.720p.x265.HEVC-LION%5BUTR%5D/Psych.S05E07.720p.x265.HEVC-LION[UTR].mkv";
    //let filepath = "file:///media/mep19mj/Anime%201/Anime/Code%20Geass%20Lelouch%20of%20the%20Rebellion/Code%20Geass%20Lelouch%20of%20the%20Rebellion%20R1%2003.mkv";
    let filepath = "file:///media/mep19mj/Anime%201/Anime/Flip%20Flappers/%20Flip%20Flappers%20-%2001.mkv";
    let pipe = format ! ("playbin uri={} name=play video-sink='autovideosink' audio-sink='autoaudiosink'", filepath);
    let mut pipeline = gstreamer::parse_launch( &pipe).unwrap();

    pipeline.set_state(gstreamer::State::Playing).expect("Unable to set to playing");

    //Using flags makes this all giga easy, we can query the names and values from the flags, hopefully to dynamically make a list of audio/sub streams
    let mut why = stream_switcher{stream: 0};
    println!("arggg {}",why.stream);
    let flags = pipeline.get_property("flags").unwrap();
    let flags_class = glib::FlagsClass::new(flags.type_()).unwrap();
    //These can then be set and unset using this value.
    let flags = flags_class.builder_with_value(flags).unwrap()
        .set_by_nick("audio")
        .build()
        .unwrap();
    pipeline.set_property("flags",&flags).unwrap();
    let bus = pipeline.get_bus().unwrap();

    //PLAYGROUND BEGINS



    //PLAYGROUND ENDS
    println!("{:?}",bus);

    let mut prev_position = 0*gstreamer::SECOND;
    let mut stream_info = _StreamInfo{
        playbin: pipeline,
        playing: false,
        terminate: false,
        seek_enabled: false,
        duration: gstreamer::CLOCK_TIME_NONE,
        n_video_streams: 0,
        n_audio_streams: 0,
        n_subtitles: 0,
        current_audio_stream:0,
        current_video_stream:0,
        current_subtitle:0,

    };

    stream_info.playbin
        .connect("audio-changed", false, |values| {
            let playbin = values[0]
                .get::<glib::Object>()
                .expect("playbin \"audio-tags-changed\" signal values[1]")
                .unwrap();

            let a = playbin.get_property("current-audio").unwrap().get_some::<i32>().unwrap();
            println!("hmm {:?}",a);

            playbin.set_property("current-audio",&1);

            None
        })
        .unwrap();



    while !stream_info.terminate {
        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not pause");
        stream_info.playbin.emit("audio-changed",&[&1]);
        let msg = bus.timed_pop( 25 * gstreamer::MSECOND);
        let keys: Vec<Keycode> = device_state.get_keys();

        match msg {
            Some(msg) => {
                handle_message(&mut stream_info, &msg);
            }
            None => {

                if keys.contains(&Keycode::Space){
                    if stream_info.playing == true{
                        println!{"Pausing"}
                        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not pause");
                    } else {
                        stream_info.playbin.set_state(gstreamer::State::Playing)
                            .expect("Could not resume");
                    }
                }

                if keys.contains(&Keycode::A) &&
                    stream_info.seek_enabled && !stream_info.playing{
                    stream_info.playbin.seek_simple(gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT, prev_position-5*gstreamer::SECOND).expect("Failed to seek");
                    prev_position = prev_position - 5*gstreamer::SECOND;

                }

                if keys.contains(&Keycode::D) &&
                    stream_info.seek_enabled && !stream_info.playing {
                    stream_info.playbin.seek_simple(gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT, prev_position + 5 * gstreamer::SECOND).expect("Failed to seek");
                    prev_position = prev_position + 5 * gstreamer::SECOND;
                }
                if stream_info.playing{

                    let mut position = stream_info
                        .playbin
                        .query_position::<gstreamer::ClockTime>().expect("Couldn't query");


                    if stream_info.duration == gstreamer::CLOCK_TIME_NONE{
                        stream_info.duration = stream_info.playbin.query_duration().expect("Couldn't query");
                    }
                    println!("Position {} / {}", position, stream_info.duration);
                    io::stdout().flush().unwrap();


                    if keys.contains(&Keycode::D) && stream_info.seek_enabled{
                        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not set to paused");
                        stream_info.playbin.seek_simple(gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT, position+5*gstreamer::SECOND).expect("Failed to seek");
                        stream_info.playbin.set_state(gstreamer::State::Playing).expect("Could not set to playing");

                    }

                    if keys.contains(&Keycode::A) && stream_info.seek_enabled {
                        stream_info.playbin.set_state(gstreamer::State::Paused).expect("Could not set to paused");
                        stream_info.playbin.seek_simple(gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::ACCURATE, position-5*gstreamer::SECOND);
                        stream_info.playbin.set_state(gstreamer::State::Playing).expect("Could not set to playing");
                    }

                    prev_position = position;
                }
            }
        }
    }
    stream_info.playbin.set_state(gstreamer::State::Null).expect("Unable to set state");



}

fn handle_message(stream_info: & mut _StreamInfo, msg: & gstreamer::Message) {
    use gstreamer::MessageView;

    match msg.view(){
        MessageView::Error(err)=>{
            println!(
                "Error received from element {:?}: {} ({:?})",
                err.get_src().map(|s| s.get_path_string()),
                err.get_error(),
                err.get_debug()
            );
            stream_info.terminate = true;
        }
        MessageView::DurationChanged(_) => {
            stream_info.duration = gstreamer::CLOCK_TIME_NONE;
        }
        MessageView::Eos(..) => {
            println!("End of stream reached.");
            stream_info.terminate = true;
        }
        MessageView::StateChanged(state_changed) => {
            if state_changed
                .get_src()
                .map(|s| s == stream_info.playbin)
                .unwrap_or(false){

                let new_state = state_changed.get_current();
                let old_state = state_changed.get_old();
                println!("State changed from {:?} to {:?}",old_state,new_state);

                stream_info.playing = new_state == gstreamer::State::Playing;
                if stream_info.playing{
                    let mut seeking = gstreamer::query::Seeking::new(gstreamer::Format::Time);
                    if stream_info.playbin.query(&mut seeking){
                        let (seekable, start, end) = seeking.get_result();
                        stream_info.seek_enabled = seekable;
                        if seekable {
                            println!("Seeking is enabled");
                        } else {
                            println!("Seeking is disabled");
                        }
                    } else {
                        eprintln!("query failed")
                    }
                }
            }
        } _ => (),
    }
}
