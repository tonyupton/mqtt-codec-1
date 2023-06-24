use  mqtt_codec::decode_packet;
use std::io::{BufWriter, Write};

mod mqtt_codec;
fn main() {
    let _packet = decode_packet();

    let mut a: u8 = 0x01;

    let b: u8 = (mqtt_codec::QoSLvl::QoS2 as u8) << 4;

    a = a + b;

    let mut buffer: [u8; 1024] = [0; 1024];
    let mut buffer_writer = BufWriter::new(&mut buffer[..]);
    buffer_writer.write(&[a]).unwrap();


    println!("Hello, world! {0:?}", _packet);
}