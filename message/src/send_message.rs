use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::rc::Rc;

use image::EncodableLayout;

use crate::message::Message;

pub struct ClojureTcpStream<T> {
    func: Rc<dyn Fn(T, Option<Vec<u8>>)>,
}

impl<T> ClojureTcpStream<T> {
    pub fn new<F>(func: F) -> ClojureTcpStream<T>
        where
            F: Fn(T, Option<Vec<u8>>) + 'static,
    {
        ClojureTcpStream {
            func: Rc::new(func),
        }
    }

    pub fn call(&self, diamond: T, data: Option<(Vec<u8>)>) {
        if data.is_some() {
            (self.func)(diamond, data);
        }
    }
}

pub fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = match std::str::from_utf8(&message_buf) {
        Err(value) => {
            println!("error {value:?}");
            exit(0)
        }
        Ok(value) => {
            println!("yes {value:?}");
            value
        }
    };
    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    record
}


fn to_stream_read_exact(stream: &mut TcpStream, buf: &mut [u8]) {
    match stream.read_exact(buf) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                println!("Failed to send data 16: {}", e);
            } else {
                println!("Failed to send data 12: {}", e);
            }
            exit(1);
        }
    };
}

pub fn read_message(stream: &mut TcpStream) -> (Option<Message>, Option<Vec<u8>>) {
    let mut total_message_size = [0; 4];
    let mut json_message_size = [0; 4];
    to_stream_read_exact(stream, &mut total_message_size);
    to_stream_read_exact(stream, &mut json_message_size);

    let total_message_size = u32::from_be_bytes(total_message_size);
    let json_message_size = u32::from_be_bytes(json_message_size);

    let mut json_data = vec![0; json_message_size as usize];
    to_stream_read_exact(stream, &mut json_data);

    let json_message = std::str::from_utf8(&json_data).expect("hello");
    let json_object = serde_json::from_str(json_message).expect("failed to serialize object");

    let data_size = total_message_size - json_message_size;


    let mut data = vec![0; data_size as usize];
    to_stream_read_exact(stream, &mut data);


    return (Some(json_object), Some(data));
}

pub fn send_message(mut stream: &mut TcpStream, message: Message, data: Option<Vec<u8>>) -> &mut TcpStream {
    let data_not_exists = data.is_none();
    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    let serialized_size_message = serialized.len() as u32;
    let data_size = match data {
        Some(ref data) => data.len() as u32,
        None => 0,
    };
    let serialized_size_total = serialized_size_message + data_size;
    let message = match message {
        Message::FragmentResult(mut result) => {
            Message::FragmentResult(result.update_offset(serialized_size_message + 32 * 2))
        }
        _ => message,
    };
    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    let serialized_size_message = serialized.len() as u32;
    let serialized_size_message_bytes = &serialized_size_message.to_be_bytes() as &[u8];
    let serialized_size_bytes = &serialized_size_total.to_be_bytes() as &[u8];
    let serialized_bytes = serialized.as_bytes();
    let compact: Vec<u8> = if data_not_exists {
        [serialized_size_bytes, serialized_size_message_bytes, serialized_bytes].concat()
    } else {
        [serialized_size_bytes, serialized_size_message_bytes, serialized_bytes, &data.clone().unwrap()].concat()
    };


    if data_not_exists {
        send_byte_with_tcp_stream(stream, Some(compact));
    } else {
        let address = String::from("localhost:8787");
        let clojure = ClojureTcpStream::new(move |address, data| {
            connect_to_server(address, compact.clone());
        });
        clojure.call(address, data);
    }
    stream
}


pub fn connect_to_server(address: String, data: Vec<u8>) {
    let stream = TcpStream::connect(address);
    match stream {
        Ok(stream) => {
            let clojure = ClojureTcpStream::new(|stream, data| {
                send_byte_with_tcp_stream(stream, data);
            });
            clojure.call(&stream, Some(data));
        }
        Err(_err) => {
            println!("Cannot connect: {}", _err);
            exit(1);
        }
    }
}

fn send_byte_with_tcp_stream(mut stream: &TcpStream, compact: Option<Vec<u8>>) {
    match compact {
        Some(compact) => {
            match stream.write_all(compact.as_bytes()) {
                Ok(_) => {
                    println!("Successfully sent data");
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::BrokenPipe {
                        println!("Failed to send data 23: {}", e);
                    } else {
                        println!("Failed to send data 123: {}", e);
                        exit(0);
                    }
                }
            };
        }
        None => {
            println!("Failed to send data 2, message is empty");
            exit(1);
        }
    }
}



