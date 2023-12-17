use std::env;
use std::io::Read;
use std::net::TcpStream;
use message::message::{FragmentRequest, FragmentResult, Message, PixelData, Resolution, U8Data};
use message::send_message::{buffer_to_object, send_message};


fn on_message_send_request(_stream: &TcpStream) {
    let fragment_request: FragmentRequest = FragmentRequest { worker_name: String::from("hello"), maximal_work_load: 10 };
    let message_send: Message = Message::FragmentRequest(fragment_request);
    send_message(_stream, message_send);
}

fn on_message_send_result(_stream: &TcpStream) {
    let u8_data: U8Data = U8Data { offset: 10, count: 10 };
    let resolution: Resolution = Resolution { nx: 10, ny: 10 };
    let pixel: PixelData = PixelData { offset: 30, count: 30 };
    let fragment_result: FragmentResult = FragmentResult { id: u8_data, resolution, pixel };
    let message_send: Message = Message::FragmentResult(fragment_result);
    send_message(_stream, message_send);
}

fn loop_message(mut _stream: &TcpStream) {
    let mut buf = [0; 4];
    on_message_send_request(_stream);
    loop {
        match _stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                println!("help");
            }
        }
        let message_size = u32::from_be_bytes(buf);
        let mut message_buf = vec![0; message_size as usize];
        _stream
            .read_exact(&mut message_buf)
            .expect("failed to read message");

        let record = buffer_to_object(&mut message_buf);
        match record {
            Message::FragmentTask(task) => {
                println!("task : {task:?}");
                on_message_send_result(_stream);
            }
            Message::FragmentRequest(request) => {
                println!("request : {request:?}");
            }
            Message::FragmentResult(result) => {
                println!("result : {result:?}");
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
    println!("{address:?}");
    let stream = TcpStream::connect(address);
    match stream {
        Ok(stream) => {
            loop_message(&stream);
        }
        Err(_err) => panic!("Cannot connect: {}", _err),
    }
}
