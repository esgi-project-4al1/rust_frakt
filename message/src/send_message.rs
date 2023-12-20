use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use image::EncodableLayout;

use crate::message::Message;
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

pub fn read_message(stream: &mut TcpStream) -> (Option<Message>, Option<Vec<u8>>) {
    let mut total_message_size = [0; 4];
    let mut json_message_size = [0; 4];
    match stream.read_exact(&mut total_message_size) {
        Ok(_) => {},
        Err(e) => {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                println!("Failed to send data: {}", e);
            } else {
                println!("Failed to send data: {}", e);
            }
            exit(1);
        }
    };
    match stream.read_exact(&mut json_message_size) {
        Ok(_) => {}
        Err(value) => {
            println!("error 2 : {:?}", value);
            exit(1);
        }
    };

    let total_message_size = u32::from_be_bytes(total_message_size);
    let json_message_size = u32::from_be_bytes(json_message_size);

    let mut json_data = vec![0; json_message_size as usize];
    stream.read_exact(&mut json_data).expect("hello");

    let json_message = std::str::from_utf8(&json_data).expect("hello");
    let json_object = serde_json::from_str(json_message).expect("failed to serialize object");

    let data_size = total_message_size - json_message_size;


    let mut data = vec![0; data_size as usize];
    stream.read_exact(&mut data).expect("hello");


    return (Some(json_object), Some(data));
}

pub fn send_message(mut stream: &TcpStream, message: Message, data: Option<Vec<u8>>) {
    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    let serialized_size_message = serialized.len() as u32;
    let data_size = match data {
        Some(ref data) => data.len() as u32,
        None => 0,
    };
    let serialized_size = serialized_size_message + data_size;

    println!("serialized_message: {:?}", serialized_size);
    let serialized = match message {
        Message::FragmentTask(_) => {
            serialized
        }
        Message::FragmentResult(mut value) => {
            let result = value.update_offset(serialized_size_message);
            let message = Message::FragmentResult(result);
            println!("serialized: {:?}", message);
            serde_json::to_string(&message).expect("failed to serialize object")
        }
        Message::FragmentRequest(_) => {
            serialized
        }
    };
    /*stream
        .write_all(&serialized_size.to_be_bytes())
        .expect("failed to send message size");

    match stream.write_all(&serialized_size_message.to_be_bytes()) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                println!("Failed to send data: {}", e);
            } else {
                println!("Failed to send data: {}", e);
                exit(0);
            }
        }
    };

    stream
        .write_all(serialized.as_bytes())
        .expect("failed to send message");

    if !data.is_none() {
        stream
            .write_all(&data.unwrap())
            .expect("failed to send data");
    }*/

    let compact_all = match data {
        Some(data) => {
            let serialized_size_bytes = &serialized_size.to_be_bytes() as &[u8];
            let serialized_size_message_bytes = &serialized_size_message.to_be_bytes() as &[u8];
            let serialized_bytes = serialized.as_bytes();
            [serialized_size_bytes, serialized_size_message_bytes, serialized_bytes, &data].concat()
        },
        None => {
            let serialized_size_bytes = &serialized_size.to_be_bytes() as &[u8];
            let serialized_size_message_bytes = &serialized_size_message.to_be_bytes() as &[u8];
            let serialized_bytes = serialized.as_bytes();
            [serialized_size_bytes, serialized_size_message_bytes, serialized_bytes].concat()
        }
    };
    match stream.write_all(&compact_all) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                println!("Failed to send data: {}", e);
            } else {
                println!("Failed to send data: {}", e);
                exit(0);
            }
        }
    };
}

