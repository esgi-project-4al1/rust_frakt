use std::io::Write;
use std::net::TcpStream;
use crate::message::Message;

pub fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");
    println!("{message:?}");
    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    record
}

#[warn(dead_code)]
fn calculate_total_message_size(json_message_size: usize, data_size: usize) -> usize {
    // Taille totale = taille du message JSON + taille des données supplémentaires
    let total_size = json_message_size + data_size;

    // Retourner la taille totale du message
    total_size
}

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    //let serialized_size = serialized.len() as u32;

    /*stream
        .write_all(serialized_size)
        .expect("failed to send serialized size");*/
    let result = stream
        .write_all(serialized.as_bytes())
        .expect("failed to send message");
    println!("{result:?}");
}


