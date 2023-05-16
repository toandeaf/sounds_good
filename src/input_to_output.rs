extern crate portaudio;

use std::collections::VecDeque;
use std::time::Instant;

use portaudio as pa;
use portaudio::stream::Buffer;
use portaudio::{Blocking, Duplex, PortAudio, Stream};

use crate::params::{get_duplex_settings, CHANNELS, FRAMES};

pub fn run() -> Result<(), pa::Error> {
    let pa = PortAudio::new()?;

    // Construct the settings with which we'll open our duplex stream.
    let duplex_settings = get_duplex_settings(&pa);

    let stream = pa.open_blocking_stream(duplex_settings)?;

    stream_loop(stream)
}

fn stream_loop(
    mut stream: Stream<Blocking<(Buffer, Buffer)>, Duplex<f32, f32>>,
) -> Result<(), pa::Error> {
    stream.start().unwrap();

    // We'll use this buffer to transfer samples from the input stream to the output stream.
    let mut buffer: VecDeque<f32> = VecDeque::with_capacity(FRAMES as usize * CHANNELS as usize);

    let start_time = Instant::now();

    loop {
        // How many frames are available on the input stream?
        let in_frames = wait_for_stream(|| stream.read_available(), "Read");

        // If there are frames available, let's take them and add them to our buffer.
        if in_frames > 0 {
            let input_samples = stream.read(in_frames)?;
            buffer.extend(input_samples.into_iter());
            // println!("Read {:?} frames from the input stream.", in_frames);
        }

        // How many frames are available for writing on the output stream?
        let out_frames = wait_for_stream(|| stream.write_available(), "Write");

        // How many frames do we have so far?
        let buffer_frames = (buffer.len() / CHANNELS as usize) as u32;

        // If there are frames available for writing and we have some to write, then write!
        if out_frames > 0 && buffer_frames > 0 {
            // If we have more than enough frames for writing, take them from the start of the buffer.
            // Otherwise if we have less, just take what we can for now.
            let write_frames = if buffer_frames >= out_frames {
                out_frames
            } else {
                buffer_frames
            };
            let n_write_samples = write_frames.clone() as usize * CHANNELS as usize;

            stream.write(write_frames, |output| {
                for i in 0..n_write_samples {
                    output[i] = buffer.pop_front().unwrap();
                }
            })?;
        }

        let elapsed_time = start_time.elapsed().as_secs();
        if elapsed_time > 5 {
            break;
        }
    }

    stream.stop().unwrap();
    stream.close()
}

// We'll use this function to wait for read/write availability.
fn wait_for_stream<F>(f: F, name: &str) -> u32
where
    F: Fn() -> Result<pa::StreamAvailable, pa::error::Error>,
{
    loop {
        match f() {
            Ok(available) => match available {
                pa::StreamAvailable::Frames(frames) => return frames as u32,
                pa::StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                pa::StreamAvailable::OutputUnderflowed => {
                    println!("Output stream has underflowed")
                }
            },
            Err(err) => panic!(
                "An error occurred while waiting for the {} stream: {}",
                name, err
            ),
        }
    }
}
