use std::net::TcpStream;
use std::process::exit;
use message::message::{FragmentRequest, Message};
use message::send_message::{read_message, send_message};

fn on_message_send_request(_stream: &mut TcpStream) {
    let fragment_request: FragmentRequest = FragmentRequest { worker_name: String::from("hello"), maximal_work_load: 1000 };
    let message_send: Message = Message::FragmentRequest(fragment_request);
    send_message(_stream, message_send, None);
}

fn on_message_send_result(_stream: &mut TcpStream, message_send: Message, data: Option<Vec<u8>>) -> &mut TcpStream {
    return  send_message(_stream, message_send, data);
}

fn loop_message(mut stream: &mut TcpStream) {
    loop {
        let (message_option, id_data) = read_message(stream);
        match message_option {
            Some(message) => {
                match message {
                    Message::FragmentTask(task) => {
                        let (fragment_result, data_result)  = task.calculate_fractal(id_data.unwrap());
                        let message_send: Message = Message::FragmentResult(fragment_result);
                        stream = on_message_send_result(stream, message_send, Some(data_result));
                    }

                    _ => {
                        println!("Is not a client message");
                        exit(200);
                    }
                }
            },
            None => {
                println!("Not a message");
                exit(0);
            }
        }
    }
}


fn main() {
    let address = String::from("localhost:8787");
    let stream = TcpStream::connect(address);
    match stream {
        Ok(mut stream) => {
            on_message_send_request(&mut stream);
            loop_message(&mut stream);
        }
        Err(_err) =>{
            println!("Cannot connect: {}", _err);
            exit(1);
        }
    }
}
