use std::env;
use std::net::TcpStream;
use std::process::exit;
use message::message::{FragmentRequest, Message};
use message::send_message::{read_message, send_message};

fn on_message_send_request(_stream: &TcpStream) {
    let fragment_request: FragmentRequest = FragmentRequest { worker_name: String::from("hello"), maximal_work_load: 1000 };
    let message_send: Message = Message::FragmentRequest(fragment_request);
    send_message(_stream, message_send, None);
}

fn on_message_send_result(_stream: &TcpStream, message_send: Message, data: Option<Vec<u8>>) {
    send_message(_stream, message_send, data);
}

fn loop_message(_stream: &mut TcpStream) {
    on_message_send_request(_stream);
    loop {
        let (message_option, data) = read_message(_stream);
        match message_option {
            Some(message) => {
                match message {
                    Message::FragmentTask(task) => {
                        let fragment_result = task.calculate_fractal();
                        let message_send: Message = Message::FragmentResult(fragment_result);
                        on_message_send_result(_stream, message_send, data);
                    }
                    Message::FragmentRequest(request) => {
                        println!("request: {:?}", request);
                    }
                    Message::FragmentResult(result) => {
                        println!("result: {:?}", result);
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
    let _args: Vec<String> = env::args().collect();
    /*let ip_address =  String::from("localhost");//String::from(&args[1]);
    let address;
    if args.len() == 4 {
        let port = String::from(&args[2]);
        address = ip_address + ":" + &port;
    } else {
        address = ip_address + ":7878";
    }*/
    let address = String::from("localhost:8787");
    let stream = TcpStream::connect(address);
    match stream {
        Ok(mut stream) => {
            loop_message(&mut stream);
        }
        Err(_err) =>{
            println!("Cannot connect: {}", _err);
            exit(1);
        }
    }
}
