
mod mqtt_codec;
fn main() {
    let mut a: u8 = 0x01;

    a = a << 1;
    
    println!("Hello, world! {0:?}", a);
}