use portaudio::PortAudio;
use tokio_stream::StreamExt;

use sounds_good::audiostream::audio_streamer_client::AudioStreamerClient;
use sounds_good::audiostream::AudioRequest;
use sounds_good::params::get_output_settings;

#[tokio::main]
async fn main() {
    let portaudio = PortAudio::new().unwrap();

    let output_settings = get_output_settings(&portaudio);

    let mut stream = portaudio.open_blocking_stream(output_settings).unwrap();

    stream.start().unwrap();

    let mut client = AudioStreamerClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    let mut client_stream = client
        .download_audio(AudioRequest {})
        .await
        .unwrap()
        .into_inner();

    while let Some(item) = client_stream.next().await {
        match item {
            Ok(chunk) => {
                let floats_vec: Vec<f32> = chunk
                    .data
                    .chunks_exact(4)
                    .map(|bytes| {
                        f32::from_ne_bytes([
                            bytes[0].clone(),
                            bytes[1].clone(),
                            bytes[2].clone(),
                            bytes[3].clone(),
                        ])
                    })
                    .collect();

                let _result = stream.write(floats_vec.len() as u32, |output| {
                    for (i, sample) in floats_vec.iter().enumerate() {
                        output[i] = *sample;
                    }
                });
            }
            Err(status) => {
                println!("Debug {}", status.message());
                break;
            }
        }
    }
}
