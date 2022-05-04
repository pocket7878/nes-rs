mod character_rom;
mod program_rom;

use character_rom::CharacterROM;
pub use program_rom::ProgramROM;

pub struct iNES {
    pub programROM: ProgramROM,
    pub characterROM: CharacterROM,
}

const HEADER_BYTES: usize = 16;
const PROGRAM_ROM_UNIT: usize = 0x4000; // 16KB
const CHARACTER_ROM_UNIT: usize = 0x2000; // 8KB
const INES_HEADER_START: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A]; // "NES" followed by MS-DOS EOF

impl iNES {
    pub fn parse(data: &[u8]) -> Result<iNES, String> {
        if data.len() < 16 {
            return Err(format!(
                "iNES header must be 16 bytes long, but was {} bytes long",
                data.len()
            ));
        }
        if data[0] != INES_HEADER_START[0]
            || data[1] != INES_HEADER_START[1]
            || data[2] != INES_HEADER_START[2]
            || data[3] != INES_HEADER_START[3]
        {
            return Err("iNES header must start with 'NES'".to_string());
        }

        eprintln!("ProgramROM Size in 16 KB units: {}", data[4]);
        eprintln!("CharacterROM Size in 8 KB units: {}", data[5]);

        let program_rom = extract_program_rom(data);
        let characte_rom = extract_character_rom(data);

        Ok(iNES {
            programROM: program_rom,
            characterROM: characte_rom,
        })
    }
}

fn extract_program_rom(data: &[u8]) -> ProgramROM {
    let program_rom_start = HEADER_BYTES;
    let program_rom_end = program_rom_start + extract_program_rom_size(data);

    ProgramROM::new(&data[program_rom_start..program_rom_end])
}

fn extract_character_rom(data: &[u8]) -> CharacterROM {
    let character_rom_size_in_unit = data[5];
    let character_rom_size = character_rom_size_in_unit as usize * CHARACTER_ROM_UNIT;
    let program_rom_size = extract_program_rom_size(data);
    let character_rom_start = HEADER_BYTES + program_rom_size;
    let character_rom_end = character_rom_start + character_rom_size;

    CharacterROM::new(&data[character_rom_start..character_rom_end])
}

fn extract_program_rom_size(data: &[u8]) -> usize {
    let program_rom_size_in_unit = data[4];
    
    program_rom_size_in_unit as usize * PROGRAM_ROM_UNIT
}
