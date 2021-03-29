use hex::encode;
use hex::decode_to_slice;
use palette::rgb::Rgb;


pub fn hex_to_color(mut hex:String) -> Option<Rgb> {
    let mut bytes = [0u8; 3]; 
    hex = hex.replace("#", "").to_string();

    match decode_to_slice(hex, &mut bytes as &mut [u8]) {
        Err(e) => {
            println!("{}", e);
            return None;  
        }, 
        Ok(_) => {
            let r = bytes[0] as f32 / 255f32;
            let g = bytes[1] as f32 / 255f32;
            let b = bytes[2] as f32 / 255f32;
            return Some (Rgb::new(r, g, b));
        }, 
    };
}

pub fn color_to_hex(color:Rgb) -> String {
    let mut bytes = [0u8; 3];
    bytes[0] = color.red as u8;
    bytes[1] = color.green as u8;
    bytes[2] = color.blue as u8;

    let mut string = encode(bytes);
    string.insert(0, '#');

    return string;
}

