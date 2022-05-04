use std::env;

use nes::{iNES, Bus, Pad, APU, CPU, DMA, PPU, RAM};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        usage(&args[0]);
        return;
    }

    let ines_rom_path = args[1].clone();
    let ines_rom = read_rom_file(&ines_rom_path);

    let ines = iNES::parse(&ines_rom).unwrap();
    eprintln!("Successfully read ines header");

    println!(
        "This ROM has {} sprites.",
        ines.characterROM.number_of_sprites()
    );

    let mut ram = RAM::new();
    let mut ppu = PPU::new();
    let mut apu = APU::new();
    let mut pad = Pad::new();
    let mut dma = DMA::new();
    let mut cpu_bus = Bus::new(
        &mut ram,
        &ines.programROM,
        &mut ppu,
        &mut apu,
        &mut pad,
        &mut dma,
    );
    let mut cpu = CPU::new(&mut cpu_bus);

    cpu.boot();
    loop {
        cpu.run_single_cycle().unwrap();
    }
}

fn read_rom_file(path: &str) -> Vec<u8> {
    use std::io::Read;

    let mut rom_file = std::fs::File::open(path).unwrap();
    let mut rom_data = Vec::new();
    rom_file.read_to_end(&mut rom_data).unwrap();
    rom_data
}

fn usage(prog_name: &str) {
    eprintln!("Usage: {} <ines>", prog_name);
}
