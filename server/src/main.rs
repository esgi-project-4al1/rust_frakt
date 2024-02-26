use std::net::{TcpListener, TcpStream};
use std::process::exit;
use message::drawing_image;

use message::message::{Message, PixelIntensity};
use message::send_message::{read_message, send_message};

use crate::fragment_task::{create_identification, RangeManager, RangeManagerTrait};
use crate::thread_pool_server::ThreadPool;

mod fragment_task;
mod thread_pool_server;

/// Transform a slice of 4 bytes to a f32
fn transform_u8_to_f32(bytes: &[u8]) -> f32 {
    return f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
}

/// Transform a chunk of 8 bytes to a PixelIntensity
fn transform_chunk_data_to_pixel_intensity(chunk_data: &[u8]) -> PixelIntensity {
    let (first_half, second_half) = chunk_data.split_at(chunk_data.len() / 2);
    return PixelIntensity {
        zn: transform_u8_to_f32(first_half),
        count: transform_u8_to_f32(second_half),
    };
}

/// Transform a vector of u8 to a vector of PixelIntensity
fn transform_data_to_vec_pixel_intensity(data: Vec<u8>) -> Vec<PixelIntensity> {
    let mut result: Vec<PixelIntensity> = Vec::new();
    for chunk in data.chunks(8) {
        result.push(transform_chunk_data_to_pixel_intensity(chunk));
    }
    return result;
}

/// Send a new fragment task to the client for the worker
#[allow(dead_code)]
fn new_fragment_task(stream: &mut TcpStream, number: u8) {
    let range = RangeManager::new();
    let fragment_task = range.get_current_range(number);
    send_message(
        stream,
        Message::FragmentTask(fragment_task),
        Some(create_identification()),
        false,
    );
}

/// Generate a new connection
#[allow(dead_code)]
fn generate_connect(_stream: &mut TcpStream, tcp_listener: &TcpListener) -> TcpStream {
    loop {
        match tcp_listener.accept() {
            Ok((new_stream, _)) => return new_stream,
            Err(e) => {
                println!("Error: {:?}", e);
                exit(1);
            }
        }
    }
}

fn loop_message(stream: &mut TcpStream, mut number: u8) {
    loop {
        let (message_option, data) = read_message(stream);
        match message_option {
            Some(message) => match message {
                Message::FragmentRequest(_fragment_request) => {
                    let fragment_task = RangeManager::new().get_current_range(number);
                    send_message(
                        stream,
                        Message::FragmentTask(fragment_task),
                        Some(create_identification()),
                        false,
                    );
                    break;
                }
                Message::FragmentResult(fragment_result) => {
                    let data_vec_pixel_intensity = match data {
                        Some(data) => data,
                        None => {
                            println!("Error: {:?}", data);
                            exit(1);
                        }
                    };
                    let test = data_vec_pixel_intensity[16..].to_vec();
                    let pixel_intensity_vec = transform_data_to_vec_pixel_intensity(test);
                    drawing_image::create_image(
                        fragment_result.resolution.nx as u32,
                        fragment_result.resolution.ny as u32,
                        &pixel_intensity_vec,
                        "fractal.png".to_string(),
                    );
                    number += 1;
                    let fragment_task = RangeManager::new().get_current_range(number);
                    send_message(
                        stream,
                        Message::FragmentTask(fragment_task),
                        Some(create_identification()),
                        false,
                    );
                    break;
                }
                _ => {
                    println!("Error: {:?}", message);
                    exit(1);
                }
            },
            _ => {
                println!("Error: {:?}", message_option);
                exit(1);
            }
        }
    }
}

/// Listen for new connections and send new fragment tasks
/// to the clients for the workers with the thread pool
/// to calculate the fractal
fn listen() {
    let listener = TcpListener::bind("localhost:8787").unwrap();

    let pool = ThreadPool::new(16);
    let number = 0;
    for stream in listener.incoming() {
        pool.execute(move || {
            println!("New connection");
            match stream {
                Ok(mut stream) => {
                    loop_message(&mut stream, number);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    exit(1);
                }
            }
        });
    }
}

/// Main function to start the server
fn main() {
    listen();
}
