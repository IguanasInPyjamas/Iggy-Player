use gstreamer;
use device_query::{DeviceQuery, DeviceState, Keycode};
use gstreamer::prelude::*;

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
    let mut prev_position = 0 * gstreamer::SECOND;
    let pipeline = uri::setup_pipeline(path);
    //TODO Implement a filepath to URI function and then this doesn't have to be hardcoded, this is just a test case.
    //TODO Use the GNU online videos for unit testing purposes.
    // TODO Implement unit tests.
    pipeline.set_state(gstreamer::State::Playing).expect("Unable to set to playing");
    let bus = pipeline.get_bus().unwrap();
    let mut stream_info = stream_info::initialise_stream(pipeline);
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
                prev_position = inputs::check_keypress(&keys, prev_position, &mut stream_info);

            }
        }
    }
    stream_info.playbin.set_state(gstreamer::State::Null).expect("Unable to set state");
}
