use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

slint::include_modules!();
fn main() {
    let ui= App::new().unwrap();
    let _ = ui.run();
    musicplay("src/assets/songs/Guitar, Loneliness and Blue Planet.flac");
    
}


fn musicplay(path: &str){
    println!("{}",path);
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
// Decode that sound file into a source
let source = Decoder::new(file).unwrap();

let sink = Sink::try_new(&stream_handle).unwrap();  
sink.append(source);
// Play the sound directly on the device
//let _player = stream_handle.play_raw(source.convert_samples());
sink.sleep_until_end();
// The sound plays in a separate audio thread,
// so we need to keep the main thread alive while it's playing.
//std::thread::sleep(std::time::Duration::from_secs(1));
}