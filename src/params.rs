use portaudio::stream::Parameters;
use portaudio::{PortAudio, StreamParameters};

const CHANNELS: i32 = 1;
const INTERLEAVED: bool = true;

pub fn get_input_params(pa: &PortAudio) -> Parameters<f32> {
    let def_input = pa.default_input_device().unwrap();
    let input_info = pa.device_info(def_input).unwrap();

    // Construct the input stream parameters.
    let latency = input_info.default_low_input_latency;
    StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency)
}

pub fn get_output_params(pa: &PortAudio) -> Parameters<f32> {
    let def_output = pa.default_output_device().unwrap();
    let output_info = pa.device_info(def_output).unwrap();

    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    println!("{}", output_info.default_sample_rate);
    StreamParameters::<f32>::new(def_output, CHANNELS, true, latency)
}
