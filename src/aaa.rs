pub fn generate_from_rom(rom: &[u8]) -> Cartridge {
    if rom[0] != b'N' || rom[1] != b'E' || rom[2] != b'S' || rom[3] != 0x1a {
        panic!("Not iNES format")
    }
    let prg_rom_size_in_16kb = rom[4] as usize;
    let chr_rom_size_in_8kb = rom[5] as usize;
    let trainer_avail = (rom[6] & 0b100) == 0b100;

    let mut prg_data: Vec<u8> = Vec::new();
    let mut chr_data: Vec<u8> = Vec::new();

    let mut data_offset: usize = if trainer_avail { 16 + 512 } else { 16 };

    for i in rom
        .iter()
        .skip(data_offset)
        .take((prg_rom_size_in_16kb * 16384) as usize)
    {
        prg_data.push(*i);
    }
    data_offset += prg_rom_size_in_16kb * 16384;

    for i in rom
        .iter()
        .skip(data_offset)
        .take((chr_rom_size_in_8kb * 8192) as usize)
    {
        chr_data.push(*i);
    }
    Cartridge {
        prg_rom_size_in_16kb: rom[4],
        chr_rom_size_in_8kb: rom[5],

        mapper_number: (rom[6] >> 4) | (rom[7] & 0b11110000),
        mirroring_control: (rom[6] & 0x1) == 1,
        battery_backed_prg_ram_available: (rom[6] & 0b10) == 0b10,
        trainer_available: trainer_avail,
        mirroring_mode: (rom[6] & 0b1000) == 0b1000,

        vs_unisystem: (rom[7] & 0b1) == 0b1,
        playchoice_10: (rom[7] & 0b10) == 0b10,
        nes_2_0_format: (rom[7] & 0b1100) == 0b1000,

        prg_ram_size_in_8kb: rom[8],

        tv_system_mode: false, // Not used for now

        prg_ram_present: (rom[10] & 0b10000) == 0b10000,
        bus_conflicts: (rom[10] & 0b100000) == 0b100000,

        prg_rom_data: prg_data,
        chr_rom_data: chr_data,
    }
}