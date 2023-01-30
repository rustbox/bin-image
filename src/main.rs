use std::io::Write;

fn main() {
    // println!("Hello, world!");

    let mut buffer: [u8; 256000] = [0; 256000];
    let width = 640;
    let height = 400;

    let image = bmp::open("image.bmp").unwrap();
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let i = (y * width + x) as usize;
            let b = rgb_bytes_to_byte((pixel.r, pixel.g, pixel.b));
            // if b == 0 {
            //     eprintln!("{}, {}, {}", pixel.r, pixel.g, pixel.b);
            // }
            buffer[i] = b;
        }
    }

    let _ = std::io::stdout().write_all(&buffer);
}

fn byte_to_color3(color: u8) -> u8 {
    color / 36
}

fn color3_to_byte(color: u8) -> u8 {
    let color = color % 8;
    match color {
        0 => 0,
        1 => 36,
        2 => 72,
        3 => 109,
        4 => 145,
        5 => 182,
        6 => 218,
        7 => 255,
        _ => 0
    }
}

/// Convert 3 channels of 3 bit red green blue into a single byte which will be understood by vgaterm
fn rgb3_to_byte(red3: u8, green3: u8, blue3: u8) -> u8 {
    // 3 bits of red green and blue, so actually 9 bits
    let r: u16 = (red3 & 0b00000111).into();
    let g: u16 = (green3 as u16 & 0b00000111 as u16) << 3;
    let b: u16 = (blue3 as u16 & 0b00000111 as u16) << 6;
    let rgb3: u16 = r + g + b;
    // Convert to vgaterm color byte by rshift 1
    (rgb3 >> 1) as u8
}

/// Given 3 channels of 8 bit red green blue, convert to a single byte which will be understood by vgaterm
fn rgb_bytes_to_byte(rgb: (u8, u8, u8)) -> u8 {
    let (r, g, b) = rgb;
    // convert 8 bit color channel to just 3 bits
    let r3 = byte_to_color3(r);
    let g3 = byte_to_color3(g);
    let b3 = byte_to_color3(b);
    
    // Red is lowest 3, green is middle 3, blue is upper 3
    rgb3_to_byte(r3, g3, b3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let rgb = (60, 22, 35);
        // 60, 22, 35
        // rgb3: (1, 0, 0)
        // assert_eq!()
    }
}
