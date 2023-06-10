use portaudio::stream::Parameters;
use portaudio::{
    DeviceIndex, DuplexStreamSettings, InputStreamSettings, OutputStreamSettings, PortAudio,
    StreamParameters, Time,
};

pub const FRAMES: u32 = 256;
pub const SAMPLE_RATE: f64 = 48000.0;
pub const FRAMES_PER_BUFFER: u32 = 1024;
pub const BITS_PER_SAMPLE: u16 = 32;
pub const OUTPUT_FILE: &str = "/Users/jaketoan/Downloads/jake.wav";
pub const CHANNELS: i32 = 1;

fn get_input_params(pa: &PortAudio) -> StreamParameters<f32> {
    let def_input = pa.default_input_device().unwrap();
    let input_info = pa.device_info(def_input).unwrap();
    let latency = input_info.default_low_input_latency;

    get_stream_params(def_input, latency)
}

fn get_output_params(pa: &PortAudio) -> StreamParameters<f32> {
    let def_output = pa.default_output_device().unwrap();
    let output_info = pa.device_info(def_output).unwrap();
    let latency = output_info.default_low_output_latency;

    get_stream_params(def_output, latency)
}

pub fn get_input_settings(pa: &PortAudio) -> InputStreamSettings<f32> {
    let stream_params = get_input_params(&pa);
    InputStreamSettings::new(stream_params, SAMPLE_RATE, FRAMES_PER_BUFFER)
}

pub fn get_output_settings(pa: &PortAudio) -> OutputStreamSettings<f32> {
    let stream_params = get_output_params(&pa);
    OutputStreamSettings::new(stream_params, SAMPLE_RATE, FRAMES_PER_BUFFER)
}

pub fn get_duplex_settings(pa: &PortAudio) -> DuplexStreamSettings<f32, f32> {
    let input_params = get_input_params(&pa);
    let output_params = get_output_params(&pa);
    DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES)
}

fn get_stream_params(device_info: DeviceIndex, latency: Time) -> Parameters<f32> {
    StreamParameters::<f32>::new(device_info, CHANNELS, true, latency)
}
