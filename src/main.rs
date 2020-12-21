
pub mod stream_info;
pub mod inputs;
pub mod events;
pub mod uri;
pub mod player;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let arg_len = args.len();
    if arg_len == 1{
        println!("No video was selected!");
    }
    else if arg_len == 2{
       player::player_loop(args);
    }
    else {
        for i in 1..(arg_len){
            let new_arg:Vec<String> = vec![args[0].clone(),args[i].clone()];
            player::player_loop(new_arg);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
