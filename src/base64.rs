use crate::err::Base64Error;

const BASE64_PAD: u8 = 0x3D;
const BASE64_CHARMAP: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn get_char_from_index(index: u8) -> Result<char, Base64Error> {
    match index {
        0..=63 => Ok(BASE64_CHARMAP[index as usize]),
        _ => Err(Base64Error::OutOfBounds),
    }
}

pub fn base64_encode<T>(input: T) -> Result<String, Base64Error>
where
    T: AsRef<str>,
{
    let mut cipher = String::new();
    let mut buffer = [0u8; 3];
    let mut count: usize = 0;
    let input_bytes = input.as_ref().as_bytes();
    (0..input_bytes.len()).into_iter().for_each(|i| {
        buffer[count] = input_bytes[i];
        count += 1;
        if count == 3 {
            let split_bits = [
                buffer[0] >> 2,
                ((buffer[0] & 0x03) << 4) | (buffer[1] >> 4),
                ((buffer[1] & 0x0F) << 2) | (buffer[2] >> 6),
                buffer[2] & 0x3F,
            ];

            for b in split_bits {
                match get_char_from_index(b) {
                    Ok(c) => {
                        cipher.push(c);
                    }
                    Err(e) => {
                        e;
                    }
                }
            }

            count = 0;
        }
    });

    /// Last group of bytes is not 3-byte long
    if count > 0 {
        let mut split_bits = [buffer[0] >> 2, 0, 0, 0];
        if count == 1 {
            split_bits[1] = (buffer[0] & 0x03) << 4;
            split_bits[2] = BASE64_PAD;
        } else if count == 2 {
            split_bits[1] = (buffer[0] & 0x03) << 4 | (buffer[1] >> 4);
            split_bits[2] = (buffer[1] & 0x0F) << 2;
        }

        split_bits[3] = BASE64_PAD;

        for b in split_bits {
            if b == BASE64_PAD {
                cipher.push('=');
            } else {
                match get_char_from_index(b) {
                    Ok(c) => {
                        cipher.push(c);
                    }
                    Err(e) => {
                        e;
                    }
                }
            }
        }
    }

    Ok(cipher)
}