use std::io::BufReader;

pub fn play_break(stream: &(rodio::OutputStream, rodio::OutputStreamHandle)) {
    play("assets/break.wav", stream)
}

pub fn play_work(stream: &(rodio::OutputStream, rodio::OutputStreamHandle)) {
    play("assets/work.wav", stream)
}

pub fn play_end(stream: &(rodio::OutputStream, rodio::OutputStreamHandle)) {
    play("assets/end.wav", stream)
}

fn play(source: &str, stream: &(rodio::OutputStream, rodio::OutputStreamHandle)) {
    use rodio::source::Source;
    let (_stream, stream_handle) = stream;
    let file = BufReader::new(std::fs::File::open(source).unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
}

pub fn create_sink() -> rodio::Sink {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    rodio::Sink::try_new(&handle).unwrap()
}

pub fn create_outputstream() -> (rodio::OutputStream, rodio::OutputStreamHandle) {
    rodio::OutputStream::try_default().unwrap()
}
