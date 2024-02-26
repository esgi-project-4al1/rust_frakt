use std::env;
use std::net::TcpStream;
use std::process::exit;

use message::message::{FragmentRequest, Message};
use message::send_message::{read_message, send_message};

/// Send the request to the server to get the fragment task
/// with the worker name and the maximal work load
fn on_message_send_request(_stream: &mut TcpStream, name: String) {
    let fragment_request: FragmentRequest = FragmentRequest {
        worker_name: name,
        maximal_work_load: 1000,
    };
    let message_send: Message = Message::FragmentRequest(fragment_request);
    send_message(_stream, message_send, None, true);
}

/// Send the result of the fragment task with the data to the server vec pixel intensity
/// transformed to vec u8
fn on_message_send_result(
    _stream: &mut TcpStream,
    message_send: Message,
    data: Option<Vec<u8>>,
) -> &mut TcpStream {
    return send_message(_stream, message_send, data, true);
}

/// Loop to read the message from the server
fn loop_message(mut stream: &mut TcpStream) {
    loop {
        let (message_option, id_data) = read_message(stream);
        match message_option {
            Some(message) => match message {
                Message::FragmentTask(task) => {
                    let id_data = match id_data {
                        Some(data) => data,
                        None => {
                            println!("Error: {:?}", id_data);
                            exit(1);
                        }
                    };
                    let (fragment_result, data_result) = task.calculate_fractal(id_data);
                    let message_send: Message = Message::FragmentResult(fragment_result);
                    stream = on_message_send_result(stream, message_send, Some(data_result));
                }
                _ => {
                    println!("Is not a client message");
                    exit(200);
                }
            },
            None => {
                println!("Not a message");
                exit(0);
            }
        }
    }
}

/// Main function to connect to the server
/// and send the request to the server
fn main() {
    let args: Vec<String> = env::args().collect();
    let name = if &args.len() >= &1 {
        String::from(&args[1])
    } else {
        String::from("hello")
    };
    let ip_address = if &args.len() >= &2 {
        String::from(&args[2])
    } else {
        String::from("localhost:8787")
    };
    let stream = TcpStream::connect(ip_address);
    match stream {
        Ok(mut stream) => {
            on_message_send_request(&mut stream, name);
            loop_message(&mut stream);
        }
        Err(_err) => {
            println!("Cannot connect: {}", _err);
            exit(1);
        }
    }
}
