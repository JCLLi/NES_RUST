//! Module that emulates a physical NES cartridge.
#[derive(PartialEq, Eq, Debug)]

/// A struct representing a physical NES cartridge which holds the contents contained in an NES file.
pub struct Cartridge {
    // TODO make these private
    // HEADER
    /// Size of program-ROM specified in multiples of 16 kB.
    pub prg_rom_size_in_16kb: u8,
    /// Size of character-ROM specified in multiples of 8 kB.
    pub chr_rom_size_in_8kb: u8,

    // Flags 6
    /// ID of used mapper (Currently supported: 0 -> NROM, 1 -> MMC1).
    pub mapper_number: u8,
    /// Indicates whether ROM from 0x8000 to 0xbfff is mirrored to 0xc0000 to 0xffff.
    pub mirroring_control: bool,
    /// Indicates whether trainer data is available.
    pub trainer_available: bool,
    /// Indicates whether cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory.
    pub battery_backed_prg_ram_available: bool,
    /// Indicates whether mirroring control is used.
    pub mirroring_mode: bool,

    // Flags 7
    // Upper nibble included in field of Flag 6
    /// Indicates whether flags 8-15 are in NES 2.0 format.
    pub nes_2_0_format: bool,
    /// Playchoice 10 supported.
    pub playchoice_10: bool,
    /// VS Unisystem supported.
    pub vs_unisystem: bool,

    // Flags 8
    /// Size of PGRM RAM specified in multiples of 8 kB.
    pub prg_ram_size_in_8kb: u8,

    // Flags 9
    /// Specifies which TV system is used (0 -> NTSC, 1 -> PAL).
    pub tv_system_mode: bool,

    // Flag 10
    /// Indicates whether PRG RAM is available.
    pub prg_ram_present: bool,
    /// Indicates whether bus conflicts are present on board.
    pub bus_conflicts: bool,

    /// Actual PRG ROM data stored in a Vec<u8>.
    pub prg_rom_data: Vec<u8>,
    /// Actual CHR ROM data stored in a Vec<u8>.
    pub chr_rom_data: Vec<u8>,
}

impl Cartridge {
    /// Creates and returns a Cartridge instance from an NES file provided as a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `rom` - A byte slice that contains the input .nes file.
    ///
    /// # Return
    /// * `Cartridge` - a Cartridge instance containing the data of `rom`.
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
}

impl Default for Cartridge {
    /// Implements the trait `Default` for Cartridge which returns a default instance of itself. All fields are set to their default state. The memory is filled with zeros.
    ///
    /// # Return
    /// * `Cartridge` - Instance of the struct in the default state.
    fn default() -> Self {
        let prg_rom_size_in_16kb = 1;
        let chr_rom_size_in_8kb = 1;
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for _i in 0..16384 * (prg_rom_size_in_16kb as u16) {
            prg_data.push(0);
        }
        for _i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            chr_data.push(0);
        }

        Cartridge {
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,

            mapper_number: 0,
            mirroring_control: false,
            battery_backed_prg_ram_available: false,
            trainer_available: false,
            mirroring_mode: false,

            vs_unisystem: false,
            playchoice_10: false,
            nes_2_0_format: false,

            prg_ram_size_in_8kb: 1,

            tv_system_mode: false, // Not used for now

            prg_ram_present: false,
            bus_conflicts: false,

            prg_rom_data: prg_data,
            chr_rom_data: chr_data,
        }
    }
}

#[cfg(test)]
mod cartridge_tests {
    use crate::cartridge::Cartridge;

    #[test]
    fn test_panic() {
        let rom: [u8; 0xffff] = [0; 0xffff]; // NOTE this ROM is only 32kb
        let panic_result = std::panic::catch_unwind(|| Cartridge::generate_from_rom(&rom));
        assert!(panic_result.is_err());
    }

    #[test]
    fn test_init_no_trainer() {
        let mut rom: [u8; 0xffff] = [0; 0xffff]; // NOTE this ROM is only 32kb

        let prg_rom_size_in_16kb = 1;
        let chr_rom_size_in_8kb = 1;
        let flag6 = 0x10; // Mapper 1 and rest set to false
        let flag7 = 0x00;
        let flag8 = 0x01; // Set RAM to 8kb
        let flag9 = 0x00;
        let flag10 = 0x00;
        let header_rom: [u8; 16] = [
            b'N',
            b'E',
            b'S',
            0x1a,
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,
            flag6,
            flag7,
            flag8,
            flag9,
            flag10,
            0x9,
            0x3E,
            0x20,
            0x00,
            0x00,
        ];
        for i in 0..16 {
            rom[i] = header_rom[i];
        }
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for i in 0..16384 * (prg_rom_size_in_16kb as u16) {
            let test_data = (i % 8).try_into().unwrap();
            prg_data.push(test_data);
            rom[(16 + i) as usize] = test_data;
        }
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            let test_data = (i % 8).try_into().unwrap();
            chr_data.push(test_data);
            rom[(16 + 16384 * (prg_rom_size_in_16kb as u16) + i) as usize] = test_data;
        }
        assert_eq!(
            Cartridge::generate_from_rom(&rom),
            Cartridge {
                prg_rom_size_in_16kb,
                chr_rom_size_in_8kb,

                mapper_number: 1,
                mirroring_control: false,
                battery_backed_prg_ram_available: false,
                trainer_available: false,
                mirroring_mode: false,

                vs_unisystem: false,
                playchoice_10: false,
                nes_2_0_format: false,

                prg_ram_size_in_8kb: 1,

                tv_system_mode: false, // Not used for now

                prg_ram_present: false,
                bus_conflicts: false,

                prg_rom_data: prg_data,
                chr_rom_data: chr_data,
            }
        );
    }

    #[test]
    fn test_init_trainer() {
        let mut rom: [u8; 0xffff] = [0; 0xffff]; // NOTE this ROM is only 32kb

        let prg_rom_size_in_16kb = 1;
        let chr_rom_size_in_8kb = 1;
        let flag6 = 0x04; // Mapper 0 with trainer
        let flag7 = 0x00;
        let flag8 = 0x01; // Set RAM to 8kb
        let flag9 = 0x00;
        let flag10 = 0x00;
        let header_rom: [u8; 16] = [
            b'N',
            b'E',
            b'S',
            0x1a,
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,
            flag6,
            flag7,
            flag8,
            flag9,
            flag10,
            0x9,
            0x3E,
            0x20,
            0x00,
            0x00,
        ];

        for i in 0..16 {
            rom[i] = header_rom[i];
        }

        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for i in 0..16384 * (prg_rom_size_in_16kb as u16) {
            let test_data = (i % 8).try_into().unwrap();
            prg_data.push(test_data);
            rom[(16 + 512 + i) as usize] = test_data;
        }
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            let test_data = (i % 8).try_into().unwrap();
            chr_data.push(test_data);
            rom[(16 + 512 + 16384 * (prg_rom_size_in_16kb as u16) + i) as usize] = test_data;
        }
        // Try the same but with trainer available (and Mapper 0)
        rom[16] = 0x04;
        assert_eq!(
            Cartridge::generate_from_rom(&rom),
            Cartridge {
                prg_rom_size_in_16kb,
                chr_rom_size_in_8kb,

                mapper_number: 0,
                mirroring_control: false,
                battery_backed_prg_ram_available: false,
                trainer_available: true,
                mirroring_mode: false,

                vs_unisystem: false,
                playchoice_10: false,
                nes_2_0_format: false,

                prg_ram_size_in_8kb: 1,

                tv_system_mode: false, // Not used for now

                prg_ram_present: false,
                bus_conflicts: false,

                prg_rom_data: prg_data,
                chr_rom_data: chr_data,
            }
        );
    }
}
