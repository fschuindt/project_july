extern crate portaudio;
extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate tls_api;

pub mod syrinx_grpc;
pub mod syrinx;

use grpc::Client;

use syrinx_grpc::*;
use syrinx::*;

use std::iter;

use portaudio as pa;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 256;
const CHANNELS: i32 = 2;
const INTERLEAVED: bool = true;

fn main() {
    tune();
    // broadcast();
    // portaudio();
}

// fn portaudio() {
//     match run_portaudio() {
//         Ok(_) => {},
//         e => {
//             eprintln!("Example failed with the following: {:?}", e);
//         }
//     }
// }

// fn run_portaudio() -> Result<(), pa::Error> {

//     let pa = try!(pa::PortAudio::new());

//     println!("PortAudio:");
//     println!("version: {}", pa.version());
//     println!("version text: {:?}", pa.version_text());
//     println!("host count: {}", try!(pa.host_api_count()));

//     let default_host = try!(pa.default_host_api());
//     println!("default host: {:#?}", pa.host_api_info(default_host));

//     let def_input = try!(pa.default_input_device());
//     let input_info = try!(pa.device_info(def_input));
//     println!("Default input device info: {:#?}", &input_info);

//     // Construct the input stream parameters.
//     let latency = input_info.default_low_input_latency;
//     let input_params = pa::StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency);

//     let def_output = try!(pa.default_output_device());
//     let output_info = try!(pa.device_info(def_output));
//     println!("Default output device info: {:#?}", &output_info);

//     // Construct the output stream parameters.
//     let latency = output_info.default_low_output_latency;
//     let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);

//     // Check that the stream format is supported.
//     try!(pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE));

//     // Construct the settings with which we'll open our duplex stream.
//     let settings = pa::DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES);

//     // Once the countdown reaches 0 we'll close the stream.
//     let mut count_down = 3.0;

//     // Keep track of the last `current_time` so we can calculate the delta time.
//     let mut maybe_last_time = None;

//     // We'll use this channel to send the count_down to the main thread for fun.
//     let (sender, receiver) = ::std::sync::mpsc::channel();

//     // A callback to pass to the non-blocking stream.
//     let callback = move |pa::DuplexStreamCallbackArgs { in_buffer, out_buffer, frames, time, .. }| {
//         let current_time = time.current;
//         let prev_time = maybe_last_time.unwrap_or(current_time);
//         let dt = current_time - prev_time;
//         count_down -= dt;
//         maybe_last_time = Some(current_time);

//         assert!(frames == FRAMES as usize);
//         sender.send(count_down).ok();

//         // Pass the input straight to the output - BEWARE OF FEEDBACK!
//         for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
//             *output_sample = *input_sample;
//             println!("{:?}", output_sample);
//         }

//         if count_down > 0.0 { pa::Continue } else { pa::Complete }
//     };

//     // Construct a stream with input and output sample types of f32.
//     let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

//     try!(stream.start());

//     // Loop while the non-blocking stream is active.
//     while let true = try!(stream.is_active()) {

//         // Do some stuff!
//         while let Ok(count_down) = receiver.try_recv() {
//             // println!("count_down: {:?}", count_down);
//         }

//     }

//     try!(stream.stop());

//     Ok(())
// }

fn tune() {
    let band_uid = String::from("rust_band_uid");
    let band_key = String::from("rust_band_key");
    let mut band = Band::new();

    band.set_uid(band_uid);
    band.set_key(band_key);

    let mut tune_req = TuneRequest::new();
    tune_req.set_band(band);

    let client = Client::new_plain("127.0.0.1", 7171, Default::default())
        .expect("Failed to initialize client");

    let tune_client = TuneClient::with_client(client);
    let tune_res = tune_client.tune(grpc::RequestOptions::new(), tune_req);

    match tune_res.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, stream)) => {
            for stream_item in stream {
                let response = stream_item.unwrap();
                println!("> {}", response.get_index());
                println!("> {:?}", response.get_chunk());
            }
        }
    }
}

fn broadcast() {
    let client = Client::new_plain("127.0.0.1", 7171, Default::default())
        .expect("Failed to initialize client");

    let iter = iter::repeat(())
        .map(|_| {
            let video_index = String::from("rust_video_index");
            let video_chunk = vec![0, 2, 4, 6];

            let mut video = Video::new();
            video.set_index(video_index);
            video.set_chunk(video_chunk);

            video
        })
        .take(13);

    let broadcast_client = BroadcastClient::with_client(client);
    // let broadcast_res = broadcast_client.broadcast(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(iter));

    let broadcast_res = broadcast_client.broadcast(grpc::RequestOptions::new(), iter);

    // println!("> {}", broadcast_res.band());
    println!("{:?}", broadcast_res.get_success);

    // let band_res = broadcast_res.get_band();

    // println!("> {}", band_res.get_uid());
    // println!("> {}", band_res.get_key());

    // match tune_res.wait() {
    //     Err(e) => panic!("{:?}", e),
    //     Ok((_, stream)) => {
    //         for stream_item in stream {
    //             let response = stream_item.unwrap();
    //             println!("> {}", response.get_index());
    //             println!("> {:?}", response.get_chunk());
    //         }
    //     }
    // }
}
