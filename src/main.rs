use  mqtt_codec::decode_packet;

use crate::mqtt_codec::ControlPacket;

mod mqtt_codec;
fn main() {
    let _packet = match decode_packet() {
        Ok(x) => x,
        Err(_) => ControlPacket::None,
    };
    println!("Hello, world! {0}", 1);
}