use rand::thread_rng;
use rand::seq::SliceRandom;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::player;

pub fn shuffle(args: &Vec<String>) -> std::vec::Vec<std::string::String>{
    let mut shuffled_list = args.clone();
    //Skip first element.
    shuffled_list[1..].shuffle(&mut thread_rng());
    return shuffled_list
}

pub fn playlist_loop(args: &Vec<String>,) {
    let mut i = 1;
    let arg_len = args.len();
    let playlist_state = DeviceState::new();
    while i < (arg_len){
        let current_stream:Vec<String> = vec![args[0].clone(),args[i].clone()];
        let previous_stream = player::player_loop(current_stream.to_vec());
        if previous_stream.shuffle{
            let new_args = shuffle(&args);
            playlist_loop(&new_args);
            //To ensure the while loop breaks after the recursed function ends.
            i += 2*arg_len;
        }

        if !previous_stream.loops{
            i += 1;

        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

pub fn playlist_check_inputs(){

}



mod tests {
    use super::*;

    #[test]
    fn shuffle_check_first_element_same() {
        let list = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(), "11".to_string(), "12".to_string(), "13".to_string(), "14".to_string(), "15".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "19".to_string(), "20".to_string()];
        let shuffled_list = shuffle(&list);
        assert_eq!(list[0], shuffled_list[0]);
    }

    #[test]
    fn check_shuffle(){
        let list = vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),"10".to_string(),"11".to_string(),"12".to_string(),"13".to_string(),"14".to_string(),"15".to_string(),"16".to_string(),"17".to_string(),"18".to_string(),"19".to_string(),"20".to_string()];
        let shuffled_list = shuffle(&list);
        assert_ne!(list[1..],shuffled_list[1..]);
    }



}
