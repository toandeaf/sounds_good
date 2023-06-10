use std::net::ToSocketAddrs;

use tonic::transport::Server;

use sounds_good::audiostream::audio_streamer_server::AudioStreamerServer;
use sounds_good::implementation::AudioStreamHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = AudioStreamHandler {};

    Server::builder()
        .add_service(AudioStreamerServer::new(handler))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
