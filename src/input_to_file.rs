use std::time::Instant;

use hound::{WavSpec, WavWriter};
use portaudio as pa;
use portaudio::stream::Buffer;
use portaudio::{Blocking, Error, Input, PortAudio, Stream, Time};

use crate::params::get_input_params;

const SAMPLE_RATE: f64 = 44100.0;
const FRAMES_PER_BUFFER: u32 = 1024;
const OUTPUT_FILE: &str = "/Users/jaketoan/Downloads/jake.wav";

pub fn run() -> Result<(), Error> {
    let portaudio = PortAudio::new().unwrap();
    let input_params = get_input_params(&portaudio);

    let mut stream: Stream<Blocking<Buffer>, Input<f32>> = portaudio
        .open_blocking_stream(pa::InputStreamSettings::new(
            input_params,
            SAMPLE_RATE,
            FRAMES_PER_BUFFER,
        ))
        .unwrap();

    stream.start().unwrap();

    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
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
