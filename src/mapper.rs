use crate::Cartridge;

#[derive(Debug, PartialEq, Eq)]
pub enum MapperType {
    Nrom { prg_rom_size_in_16kb: u8 },
}
impl Default for MapperType {
    fn default() -> Self {
        MapperType::Nrom {
            prg_rom_size_in_16kb: 1,
        }
    }
}
impl MapperType {
    pub fn get_mapper(mapper_number: u8, cartridge: Cartridge) -> MapperType {
        match mapper_number {
            0 => MapperType::Nrom {
                prg_rom_size_in_16kb: cartridge.prg_rom_size_in_16kb,
            },
            _ => panic!("Mapper not implemented/known"),
        }
    }
    pub fn get_mapper_address(&self, addr: u16) -> u16 {
        match self {
            MapperType::Nrom {
                prg_rom_size_in_16kb,
            } => {
                if addr < 0x8000 {
                    return addr;
                }
                if *prg_rom_size_in_16kb > 2 {
                    panic!("Program ROM is too big for NROM")
                }
                if *prg_rom_size_in_16kb == 2 {
                    return addr;
                }
                if addr < 0xc000 {
                    addr
                } else {
                    addr - 16384
                }
            } //_ => panic!("Mapper not implemented yet")
        }
    }
    pub fn write_mapper(&self, _prg_rom_size: u8, _addr: u16, _data: u8) {
        //You can't write to read-only memory
    }
}

#[cfg(test)]
mod mapper_tests {
    use crate::MapperType::Nrom;
    use crate::{Cartridge, MapperType};

    #[test]
    fn test_nrom() {
        let cart = Cartridge::default();
        let mapper0 = MapperType::get_mapper(0, cart);
        assert_eq!(
            mapper0,
            Nrom {
                prg_rom_size_in_16kb: 1
            }
        );
        assert_eq!(mapper0.get_mapper_address(0xC000), 0x8000);
        // Make cartridge with 32 kb
        let cart_32kb = Cartridge {
            prg_rom_size_in_16kb: 2,
            chr_rom_size_in_8kb: 1,
            mapper_number: 0,
            mirroring_control: false,
            trainer_available: false,
            battery_backed_prg_ram_available: false,
            mirroring_mode: false,
            nes_2_0_format: false,
            playchoice_10: false,
            vs_unisystem: false,
            prg_ram_size_in_8kb: 1,
            tv_system_mode: false,
            prg_ram_present: false,
            bus_conflicts: false,
            prg_rom_data: Vec::new(), // Dont need this
            chr_rom_data: Vec::new(), // Don't need this
        };
        let mapper0_32kb = MapperType::get_mapper(0, cart_32kb);
        assert_eq!(mapper0_32kb.get_mapper_address(0x8000), 0x8000);
        assert_eq!(mapper0_32kb.get_mapper_address(0xC000), 0xC000);
    }
}
