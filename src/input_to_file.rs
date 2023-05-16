use std::time::Instant;

use hound::{WavSpec, WavWriter};
use portaudio::stream::Buffer;
use portaudio::{Blocking, Error, Input, PortAudio, Stream};

use crate::params::{
    get_input_settings, BITS_PER_SAMPLE, CHANNELS, FRAMES_PER_BUFFER, OUTPUT_FILE, SAMPLE_RATE,
};

pub fn run() -> Result<(), Error> {
    let portaudio = PortAudio::new().unwrap();
    let input_settings = get_input_settings(&portaudio);

    let mut stream: Stream<Blocking<Buffer>, Input<f32>> =
        portaudio.open_blocking_stream(input_settings).unwrap();

    stream.start().unwrap();

    let spec = WavSpec {
        channels: CHANNELS as u16,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Float,
    };

    let mut output_file = WavWriter::create(OUTPUT_FILE, spec).unwrap();

    let time = Instant::now();

    for _ in 0..(SAMPLE_RATE as usize * 5) {
        let buffer: Vec<f32> = stream.read(FRAMES_PER_BUFFER)?.to_vec();
        for &sample in buffer.iter() {
            match output_file.write_sample(sample.clone()) {
                Ok(_result) => {}
                Err(err) => {
                    println!("{}", err)
                }
            };
        }

        if time.elapsed().as_secs() > 3 {
            break;
        }
    }

    output_file.finalize().unwrap();

    stream.stop().unwrap();
    stream.close().unwrap();

    portaudio.terminate()
}
