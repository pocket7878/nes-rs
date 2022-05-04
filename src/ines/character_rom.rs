pub struct CharacterROM {
    data: Vec<u8>,
}

const SINGLE_SPRITE_BYTES: usize = 16;

impl CharacterROM {
    pub fn new(data: &[u8]) -> CharacterROM {
        CharacterROM {
            data: data.to_vec(),
        }
    }

    pub fn nth_sprite(&self, n: usize) -> Sprite {
        let start = n * SINGLE_SPRITE_BYTES;
        let end = start + SINGLE_SPRITE_BYTES;

        let mut sprite_bytes: [u8; 16] = Default::default();
        sprite_bytes.copy_from_slice(&self.data[start..end]);

        Sprite::new(sprite_bytes)
    }

    pub fn number_of_sprites(&self) -> usize {
        self.data.len() / SINGLE_SPRITE_BYTES
    }
}

pub struct Sprite {
    lower_half_bytes: [u8; 8],
    upper_half_bytes: [u8; 8],
}

impl Sprite {
    pub fn new(bytes: [u8; 16]) -> Sprite {
        Sprite {
            lower_half_bytes: [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ],
            upper_half_bytes: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }

    pub fn pallet_color_number_for_pixel(&self, x: usize, y: usize) -> Result<u8, String> {
        if x >= 8 {
            return Err("x must be between 0 and 7".to_string());
        }

        if y >= 8 {
            return Err("y must be between 0 and 7".to_string());
        }

        let bit_index = 7 - x;
        let byte_index = y;

        let lower_half_byte = self.lower_half_bytes[byte_index];
        let lower_half_bit = lower_half_byte >> bit_index;
        let lower_half_bit = lower_half_bit & 1;

        let upper_half_byte = self.upper_half_bytes[byte_index];
        let upper_half_bit = upper_half_byte >> bit_index;
        let upper_half_bit = upper_half_bit & 1;

        Ok(lower_half_bit + upper_half_bit)
    }
}
