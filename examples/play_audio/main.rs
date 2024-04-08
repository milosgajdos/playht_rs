//! `cargo run --example play_audio -- "/path/to/audio.mp3"`
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

fn main() {
    let mut args = std::env::args().skip(1);
    let sound_file = args.next().unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(&sound_file).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
