pub mod stream_info;
pub mod inputs;
pub mod events;
pub mod uri;
pub mod player;
pub mod playlists;
pub mod window_processing;

fn main() {
    let mut args:Vec<String> = std::env::args().collect();
    let arg_len = args.len();
    if arg_len == 1{
        println!("No video was selected!");
    }
    else if arg_len == 2{
        player::player_loop(args);
    }
    else {
        playlists::playlist_loop(&args);

    }
}
