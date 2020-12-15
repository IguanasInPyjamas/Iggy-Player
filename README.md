# Iggy-Player
A lightweight media player in rust using gstreamer.
Nowhere near to completion - collaboration is welcome!

At the moment the video file needs to be set as a URI in the filepath variable in src/main.rs for testing.

TODO;
- Fix panic of seeking when the video is playing 
- Add a filepath to uri converter and then pass argc and argv to main
- Handling of subtitle / multiple audio tracks
- Playlists 
- Shuffling, loop 
