use hound::WavReader;
use portaudio::{Error, PortAudio};

use crate::params::{get_output_settings, FRAMES_PER_BUFFER, OUTPUT_FILE};

pub fn run() -> Result<(), Error> {
    let portaudio = PortAudio::new().unwrap();
    let mut reader = WavReader::open(OUTPUT_FILE).unwrap();

    let output_settings = get_output_settings(&portaudio);

    let mut stream = portaudio.open_blocking_stream(output_settings).unwrap();
    stream.start()?;

    let mut samples: Vec<f32> = reader
        .samples::<f32>()
        .map(|sample| sample.unwrap())
        .collect();

    while !samples.is_empty() {
        let frames_to_write = std::cmp::min(samples.len(), FRAMES_PER_BUFFER as usize);
        let to_write = samples.drain(..frames_to_write).collect::<Vec<f32>>();

        stream.write(to_write.len() as u32, |output| {
            for (i, sample) in to_write.iter().enumerate() {
                output[i] = *sample;
            }
        })?;
    }

    stream.stop()?;
    stream.close()?;
    Ok(())
}
