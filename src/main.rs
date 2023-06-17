use  mqtt_codec::decode_packet;

mod mqtt_codec;
fn main() {
    let packet = match decode_packet() {
        Ok(x) => x,
        Err(_) => panic!(),
    };
    println!("Hello, world! {0}", 1);
}
