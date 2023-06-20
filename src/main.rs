use  mqtt_codec::decode_packet;

mod mqtt_codec;
fn main() {
    let _packet = decode_packet();
    println!("Hello, world! {0}", 1);
}