use pretty_hex::*;

fn main() {
    let data = vec![0x27, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30];
    println!("{:?}", data.hex_dump());
}
