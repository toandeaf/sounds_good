use crate::params::get_output_params;
use hound::WavReader;
use portaudio::{Error, OutputStreamSettings, PortAudio};

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 1024;
const OUTPUT_FILE: &str = "/Users/jaketoan/Downloads/jake.wav";

pub fn run() -> Result<(), Error> {
    let portaudio = PortAudio::new().unwrap();
    let mut reader = WavReader::open(OUTPUT_FILE).unwrap();

    let output_params = get_output_params(&portaudio);
    let output_settings = OutputStreamSettings::new(output_params, SAMPLE_RATE, FRAMES_PER_BUFFER);

    let mut stream = portaudio.open_blocking_stream(output_settings).unwrap();
    stream.start()?;

    let mut samples: Vec<f32> = reader
        .samples::<f32>()
        .map(|sample| sample.unwrap())
        .collect();

    while !samples.is_empty() {
        let frames_to_write = std::cmp::min(samples.len(), FRAMES_PER_BUFFER as usize);
        let mut to_write = samples.drain(..frames_to_write).collect::<Vec<f32>>();
        stream.write(to_write.len() as u32, |output| {
            for i in 0..to_write.len() {
                output[i] = to_write.pop().unwrap();
                println!("{:?}", output);
            }
        })?;
    }

    stream.stop()?;
    stream.close()?;
    Ok(())
}
