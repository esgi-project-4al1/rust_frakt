use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::rc::Rc;

use crate::message::Message;

pub struct ClojureTcpStream<T> {
    func: Rc<dyn Fn(T, Option<Vec<u8>>)>,
}

/// A ClojureTcpStream is a wrapper around a TcpStream that allows you to pass a closure
/// to be called when data is received from the stream.
/// The closure is called with the TcpStream and the received data.
/// Is just a test for generic type
impl<T> ClojureTcpStream<T> {
    pub fn new<F>(func: F) -> ClojureTcpStream<T>
    where
        F: Fn(T, Option<Vec<u8>>) + 'static,
    {
        ClojureTcpStream {
            func: Rc::new(func),
        }
    }

    pub fn call(&self, diamond: T, data: Option<Vec<u8>>) {
        if data.is_some() {
            (self.func)(diamond, data);
        }
    }
}

/// read the message from the stream
/// and return the message and the data
/// if the message is not a message
/// then exit the program
pub fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = match std::str::from_utf8(&message_buf) {
        Err(value) => {
            println!("error {value:?}");
            exit(0)
        }
        Ok(value) => {
            println!("yes deux {value:?}");
            value
        }
    };
    match serde_json::from_str(&message) {
        Err(value) => {
            println!("Failed to deserialize JSON message {value:?}");
            exit(0)
        }
        Ok(value) => {
            println!("yes trois {value:?}");
            value
        }
    }
}

/// Read the exact data from the stream
/// and handle the error
/// if the stream is broken
/// then exit the program
fn to_stream_read_exact(stream: &mut TcpStream, buf: &mut [u8]) {
    match stream.read_exact(buf) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                println!("Failed to send data 16: {}", e);
            } else {
                println!("Failed to send data 12: {} {}", e, buf.len());
            }
            exit(1);
        }
    };
}

/// Read a message with a TcpStream and return the message and the data
/// if the message is not a message then exit the program
/// if the stream is broken then exit the program
/// if the data is not exists then return the message else return the message with the data
/// if the client is true then connect to the server and read the message with new stream
pub fn read_message(stream: &mut TcpStream) -> (Option<Message>, Option<Vec<u8>>) {
    let mut total_message_size = [0; 4];
    let mut json_message_size = [0; 4];
    to_stream_read_exact(stream, &mut total_message_size);
    to_stream_read_exact(stream, &mut json_message_size);

    let total_message_size = u32::from_be_bytes(total_message_size);
    let json_message_size = u32::from_be_bytes(json_message_size);

    let mut json_data = vec![0; json_message_size as usize];
    to_stream_read_exact(stream, &mut json_data);

    let json_message =
        std::str::from_utf8(&json_data).expect("Failed to convert JSON data to string");
    let json_object =
        serde_json::from_str(json_message).expect("Failed to deserialize JSON message");

    let data_size = total_message_size - json_message_size;

    let mut data = vec![0; data_size as usize];
    to_stream_read_exact(stream, &mut data);

    (Some(json_object), Some(data))
}

/// Send a message with a TcpStream
/// and handle the error
/// if the stream is broken
/// then exit the program
/// with a message
/// if the client is true
/// then connect to the server
/// and send the message with new stream
/// if the data is not exists then send the message else send the message with the data
pub fn send_message(
    stream: &mut TcpStream,
    message: Message,
    data: Option<Vec<u8>>,
    client: bool,
) -> &mut TcpStream {
    let data_not_exists = data.is_none();
    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    let serialized_size_message = serialized.len() as u32;
    let data_size = data.as_ref().map(|data| data.len() as u32).unwrap_or(0);
    let serialized_size_total = serialized_size_message + data_size;
    let serialized_size_message_bytes = &serialized_size_message.to_be_bytes() as &[u8];
    let serialized_size_bytes = &serialized_size_total.to_be_bytes() as &[u8];
    let serialized_bytes = serialized.as_bytes();

    let compact: Vec<u8> = if data_not_exists {
        [
            serialized_size_bytes,
            serialized_size_message_bytes,
            serialized_bytes,
        ]
        .concat()
    } else {
        if let Some(data) = &data {
            [
                serialized_size_bytes,
                serialized_size_message_bytes,
                serialized_bytes,
                data,
            ]
            .concat()
        } else {
            [
                serialized_size_bytes,
                serialized_size_message_bytes,
                serialized_bytes,
            ]
            .concat()
        }
    };

    if data_not_exists {
        send_byte_with_tcp_stream(stream, Some(compact));
        return stream;
    } else if client == false {
        send_byte_with_tcp_stream(stream, Some(compact));
        return stream;
    } else {
        let address = "localhost:8787".to_string();
        match connect_to_server(address) {
            Ok(server) => {
                send_byte_with_tcp_stream(server, Some(compact));
                return server;
            }
            Err(err) => {
                println!("{}", err);
                exit(1);
            }
        }
    }
}

/// Display the data as hex for debugging server
pub fn display_data(data: Vec<u8>) {
    for i in 0..data.len() {
        println!("Byte value as hex: {:#02x}", data[i]);
    }
    println!();
}

/// Connect to a server is a helper function
pub fn connect_to_server(address: String) -> Result<&'static mut TcpStream, String> {
    match TcpStream::connect(address) {
        Ok(stream) => {
            let boxed_stream = Box::new(stream);
            let stream_ref = Box::leak(boxed_stream);
            Ok(stream_ref)
        }
        Err(err) => Err(format!("Cannot connect: {}", err)),
    }
}

/// Send a byte with a TcpStream
/// and handle the error
/// if the stream is broken
/// then exit the program
/// with a message
fn send_byte_with_tcp_stream(mut stream: &TcpStream, compact: Option<Vec<u8>>) {
    match compact {
        Some(compact) => {
            let mut offset = 0;
            while offset < compact.len() {
                match stream.write(&compact[offset..]) {
                    Ok(n) => {
                        offset += n;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::BrokenPipe {
                            println!("Failed to send data Broken Pipe: {}", e);
                        } else {
                            println!("Failed to send data: {}", e);
                            exit(0);
                        }
                    }
                }
            }
        }
        None => {
            println!("Failed to send data 2, message is empty");
            exit(1);
        }
    }
}
