pub mod audiostream {
    tonic::include_proto!("audiostream"); // The string specified here must match the proto package name
}

pub mod implementation;
pub mod params;
