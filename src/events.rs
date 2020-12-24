use gstreamer;
use gstreamer::prelude::*;
use glib;

use crate::stream_info;

pub fn setup_flags_class(stream_info: & stream_info::_StreamInfo) -> (glib::Value ,glib::FlagsClass){
    let flags = stream_info.playbin.get_property("flags").unwrap();
    let flags_class = glib::FlagsClass::new(flags.type_()).unwrap();
    return (flags,flags_class);

}

pub fn setup_connects(stream_info: & stream_info::_StreamInfo){
    stream_info.playbin
        .connect("audio-changed", false, |values| {
            let playbin = values[0]
                .get::<glib::Object>()
                .expect("playbin \"audio-tags-changed\" signal values[1]")
                .unwrap();

            let a = playbin.get_property("current-audio").unwrap().get_some::<i32>().unwrap();
            None
        })
        .unwrap();

    stream_info.playbin
        .connect("audio-tags-changed", false, |values| {
            let playbin = values[0]
                .get::<glib::Object>()
                .expect("playbin \"audio-changed\" signal values[1]")
                .unwrap();

            let idx = values[1]
                .get_some::<i32>()
                .expect("playbin \"audio-changed\" signal values[1]");

            let tags = playbin.emit("get-audio-tags", &[&idx]).unwrap().unwrap();
            let tags = tags.get::<gstreamer::TagList>().expect("tags").unwrap();

            if let Some(artist) = tags.get::<gstreamer::tags::LanguageCode>() {
                println!("  Language changed to : {}", artist.get().unwrap());
            }

            None
        })
        .unwrap();
}

pub fn handle_message(stream_info: &mut stream_info::_StreamInfo, msg: & gstreamer::Message) {
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
                        let (seekable, _start, _end) = seeking.get_result();
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
