use std::pin::Pin;

use portaudio::stream::CallbackResult;
use portaudio::PortAudio;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tonic::{Request, Response, Status, Streaming};

use crate::audiostream::audio_streamer_server::AudioStreamer;
use crate::audiostream::{AudioChunk, AudioRequest, UploadStatus};
use crate::params::get_input_settings;

#[derive(Default)]
pub struct AudioStreamHandler;

type AudioResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<AudioChunk, Status>> + Send>>;

#[tonic::async_trait]
impl AudioStreamer for AudioStreamHandler {
    async fn upload_audio(
        &self,
        request: Request<Streaming<AudioChunk>>,
    ) -> AudioResult<UploadStatus> {
        let mut stream = request.into_inner();

        // Loop over stream to process audio chunks
        while let Some(_audio_chunk) = stream.message().await? {
            // TODO: Process audio chunk
        }

        let reply = UploadStatus {
            message: "Audio upload completed!".into(),
        };

        Ok(Response::new(reply))
    }

    type DownloadAudioStream = ResponseStream;

    async fn download_audio(
        &self,
        req: Request<AudioRequest>,
    ) -> Result<Response<Self::DownloadAudioStream>, Status> {
        println!("\tclient connected from: {:?}", req.remote_addr());

        let (sender, receiver) = mpsc::channel(128);

        let pa = PortAudio::new().unwrap();
        let input_settings = get_input_settings(&pa);
        let output_stream = ReceiverStream::new(receiver);

        let mut stream = pa
            .open_non_blocking_stream(input_settings, move |data| {
                let mut overall_result: CallbackResult = CallbackResult::Continue;
                for sample in data.buffer.iter() {
                    // TODO Why in the name of christ does this callback only invoke if i have this here lmao
                    // println!("Am i losing it?");

                    let small_bytes = sample.to_ne_bytes();
                    let result =
                        sender.blocking_send(Result::<AudioChunk, Status>::Ok(AudioChunk {
                            data: small_bytes.to_vec(),
                        }));
                    match result {
                        Ok(_) => CallbackResult::Continue,
                        Err(e) => {
                            overall_result = CallbackResult::Abort;
                            break;
                        }
                    };
                }
                overall_result
            })
            .unwrap();

        stream.start().unwrap();

        tokio::spawn(async move {
            while stream.is_active().unwrap() {
                tokio::task::yield_now().await;
            }
            println!("\tclient disconnected");
        });

        Ok(Response::new(
            Box::pin(output_stream) as Self::DownloadAudioStream
        ))
    }
}
